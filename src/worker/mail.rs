use std::result;
use std::sync::Arc;
use lapin::Consumer;
use crate::configuration::api_state::SharedState;
use futures_util::StreamExt;
use lapin::options::{BasicAckOptions, BasicNackOptions};
use tokio::spawn;
use crate::service::{
    worker::deserialize_message,
    template_render::template_render,
};
use crate::repository::notification::{db_get_notification, db_get_notification_retry_count, db_update_notification_retry_count, db_update_notification_status};

pub async fn process_each_message_mail(state: SharedState,mut consumer: Consumer){
    println!("message processing started");
    // loop will end when we get None
    // we get none when consumer stream ends
    while let Some(msg) = consumer.next().await {
        match msg {
            Ok(mut msg) => {
                println!("message recieved`");

                // creating owner
                let state = Arc::clone(&state);

                // spawn new task to process message
                spawn(async {
                    // getting state
                    let state = state;

                    // getting message
                    let mut message = msg;

                    // deserialize message to struct and get notification_id
                    let notfication_id = match deserialize_message(&(message.data)) {
                        Ok(notif) => notif.notification_id,
                        Err(e) => {
                            println!("{}",e);
                            return;
                        }
                    };

                    // fetch get notification for Db
                   let notification =  match db_get_notification(&(*state).db_pool,
                                                                 notfication_id).await {
                       Ok(notification) => notification,
                       Err(e) => {
                           println!("{}",e);
                           return;
                       }
                   };
                    println!("{:?}",notification);

                    // check if notification is sent again by rbmq
                    if message.redelivered {
                        // update retry count by 1
                        let current_retry_count = notification.retry_count;
                        let result= db_update_notification_retry_count(&(*state).db_pool,
                                                                        notfication_id,
                                                                        current_retry_count + 1)
                            .await
                            .map_err(|e| {
                                println!("{}",e);
                                return;
                            });
                    }

                    // get template
                    let template = match (*state).template_cache.get_template_mail(&(*state).db_pool,
                                          &notification.template,
                                             &notification.channel,
                    ).await {
                        Ok(template) => {
                            template
                        },
                        Err(e) => {
                            return;
                        }
                    };


                    // render template
                    let (subject, body) = template_render(template,&notification.variables);


                    // send mail
                    let mail_sent = (*state).email_service.send(
                        notification.recipient.as_str(),
                        subject.as_str(),
                        body.as_str(),
                    ).await;

                    match mail_sent {
                        // mail sent
                        Ok(_) => {
                            println!("mail sent");

                            // acknowledge message
                            message.acker.ack(BasicAckOptions::default()).await;

                            // update in db that mail is sent
                            let result = db_update_notification_status(
                                &(*state).db_pool,
                                notification.id,
                                "SENT",
                            ).await;

                            if let Err(e) = result {
                                println!("notification status update failed : {}",e);
                                return;
                            }
                            return;

                        },
                        // mail failed to sent
                        Err(e) => {
                            println!("mail failed to sent : {}", e);
                            // retry count
                            let retry_count =  match db_get_notification_retry_count(
                                &(*state).db_pool,
                                notification.id
                            ).await {
                                Ok(c) => c,
                                Err(e) => {
                                    println!("{}",e);
                                    return;
                                }
                            };

                            // if its last retry
                            if retry_count == 3 {
                                // -ve acknowledge message and requeue
                                let nack = message.acker.nack(BasicNackOptions{
                                    multiple: false,
                                    requeue: false,
                                }).await;

                                // update in db that mail sent is failed
                                let result = db_update_notification_status(
                                    &(*state).db_pool,
                                    notification.id,
                                    "FAILED",
                                ).await;

                                if let Err(e) = result {
                                    println!("notification status update failed : {}",e);
                                    return;
                                }

                            }else { // we have more retry
                                // -ve acknowledge message and requeue
                                let nack = message.acker.nack(BasicNackOptions{
                                    multiple: false,
                                    requeue: true,
                                }).await;

                                // update in db that mail sent is retrying
                                let result = db_update_notification_status(
                                    &(*state).db_pool,
                                    notification.id,
                                    "RETRYING",
                                ).await;

                                if let Err(e) = result {
                                    println!("notification status update failed : {}",e);
                                    return;
                                }
                            }


                        },
                    }

                });
            }

            Err(e) => {
                println!("{}",e);
                continue;
            }
        }
    }

}