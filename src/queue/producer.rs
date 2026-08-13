use std::collections::HashMap;
use axum::{
    http::StatusCode,
};
use lapin::{Connection, ConnectionProperties, Channel, options::{BasicPublishOptions, QueueDeclareOptions}, BasicProperties, Consumer};
use lapin::options::{BasicConsumeOptions, QueueBindOptions, QueueDeleteOptions};
use lapin::types::FieldTable;
use uuid::Uuid;
use crate::datamodels::rbmq::notifation::NotifMsg;
use crate::error::AppError;

pub struct QueueConn {
    conn: Connection,
    exchange_name: String,
    routing_keys: std::collections::HashMap<&'static str,String>
}


impl QueueConn {
    // creates new connection
    pub async fn new() -> QueueConn {
        // rabbitmq connection string
        let url = std::env::var("RABBITMQ_URL")
            .expect("environment variable RABBITMQ_URL is not present");

        // exchange name
        let exchange_name = std::env::var("RABBITMQ_EXCHANGE")
            .expect("environment variable RABBITMQ_EXCHANGE is not present");

        // routing keys
        let routing_keys = Self::load_all_routing_keys();

        let mut conn = Connection::connect(url.as_str(), ConnectionProperties::default())
            .await
            .expect("failed to connect to rabbitmq");

        QueueConn{
            conn: conn,
            exchange_name,
            routing_keys
        }

    }
    fn load_all_routing_keys() -> HashMap<&'static str,String> {
        let mut m = HashMap::new();

        //
        m.insert("MAIL",
                 std::env::var("RABBITMQ_ROUTING_KEY_MAIL")
                     .expect("environment variable RABBITMQ_ROUTING_KEY_MAIL is not present"));

        m
    }

    // creates new channel
    async fn ch(&self) -> Result<Channel,AppError> {
        self.conn.create_channel()
            .await
            .map_err(|e|{
                tracing::error!("queue : {:?}",e);
                AppError::Queue(e)
            }
            )
    }

    // publish message to rabbitmq exchange
    pub async fn publish(&self, notif_id: Uuid, channel_type: &String) -> Result<(),AppError>{
        // creating channel
        let ch = self.ch().await?;

        // creating message
        let msg = NotifMsg {
            notification_id: notif_id
        };

        // serializing message  Struct -> Json String -> Bytes
        let json_msg = serde_json::to_string(&msg)
            .map_err(|e|{
                tracing::error!("queue : {:?}",&e);
                AppError::Serialization(e)
            })?;
        let json_bytes = json_msg.as_bytes(); // message

        //publishing message to exchange
        let routing_key = (&self.routing_keys).get(channel_type.as_str()).unwrap();
        ch.basic_publish(
            self.exchange_name.as_str(),
            routing_key.as_str(),
            BasicPublishOptions{
                mandatory: false,
                immediate: false,
            },
            json_bytes,
            BasicProperties::default()
        ).await
            .map_err(|e|{
                tracing::error!("queue : {:?}",&e);
                AppError::Queue(e)
            })?;
        // exchange name is wrong still function not getting failed need to resolve it

        return Ok(())
    }

    // creates queue
    // created queue will, durable, non-exclusive, auto-delete: false
    pub async fn create_queue_and_bind(
        &self,
        queue_name: String,
        worker_type: &str, // "MAIL", "SMS"
    ) -> Result<lapin::Queue,AppError> {
        // create channel
        let channel_for_checking = self.ch().await?; // if queue not exits channel will be closed

        // checking if already exits queue
        let queue = channel_for_checking.queue_declare(
            &queue_name,
            QueueDeclareOptions{
                passive: true , // it will check if queue exits, if exits it Ok()
                // if not exist channel get closed, and err is return no queue created
                durable: true, // so queue lives even after restart
                exclusive: false, // queue can be access by other connection too
                auto_delete: false, // if last connection to queue closes it not get deleted
                nowait: false, // no fire-and-forget
            },
            FieldTable::default()
        ).await;

        // true = queue exits // false = queue not exits
        let queue_exits  =  queue.is_ok();

        // queue not exits , create queue
        if !queue_exits {
            let channel = self.ch().await?; // channel for creating queue and binding

            // create queue
            let queue = channel.queue_declare(
                &queue_name,
                QueueDeclareOptions{
                    passive: false , // it will not check, it will create queue
                    // if not exist channel get closed, and err is return no queue created
                    durable: true, // so queue lives even after restart
                    exclusive: false, // queue can be access by other connection too
                    auto_delete: false, // if last connection to queue closes it not get deleted
                    nowait: false, // no fire-and-forget
                },
                FieldTable::default()
            ).await?;

            // routing key
            let routing_key = if let Some(key) = self.routing_keys.get(worker_type) {
                key
            }else {
                return Err(AppError::RoutingKeyNotExist);
            };

            // binding queue to exchange
            let res = channel.queue_bind(
                queue.name().as_str(),
                &self.exchange_name,
                routing_key,
                QueueBindOptions{
                    nowait: false,
                },
                FieldTable::default()
            ).await;

            // if binding fails delete that queue
            if let Err(e) = res {
                // deleting queue
                if let Err(e) = channel.queue_delete(queue.name().as_str(),QueueDeleteOptions{
                    if_unused: false,
                    if_empty: false,
                    nowait: false
                }).await {
                    return Err(AppError::Queue(e))
                }
                
                return Err(AppError::Queue(e))
            }

            return Ok(queue)
        }

        Ok(queue.unwrap())

    }

    // creating consumer, after creating this consumer no other consumer can access same queue
    pub async fn create_consume(&self, queue_name: String) -> Result<Consumer,AppError>{
        // creating consumer
        let channel = self.ch().await?;

        //creating consumer
        let consumer = channel.basic_consume(
            &queue_name,
            "",
            BasicConsumeOptions{
            no_local: false,
            no_ack: false, // message ack manually, no auto ack
            exclusive: true, // now only current consumer can access the queue
            nowait: false, // no fire and forget
        },
            FieldTable::default(),
        ).await?;

        Ok(consumer)
    }
}