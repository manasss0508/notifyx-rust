use std::sync::Arc;
use lapin::Consumer;
use crate::configuration::api_state::SharedState;
use futures_util::StreamExt;
use lapin::options::BasicAckOptions;
use tokio::spawn;
use crate::service::{
    worker::deserialize_message,
    template_render::template_render,
};
use crate::repository::notification::db_get_notification;

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

                    println!("subject: {}, \
                    body: {}",subject,body);

                    // send mail

                    // acknowledge message
                    message.acker.ack(BasicAckOptions::default()).await;


                });
            }

            Err(e) => {
                println!("{}",e);
                continue;
            }
        }
    }

}