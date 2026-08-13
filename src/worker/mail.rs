use lapin::Consumer;
use crate::configuration::api_state::SharedState;
use crate::error::AppError;
use futures_util::StreamExt;

pub async fn process_each_message_mail(state: SharedState,mut consumer: Consumer){

    // loop will end when we get None
    // we get none when consumer stream ends
    while let Some(msg) = consumer.next().await {
        match msg {
            Ok(msg) => {println!("message received: {:?}", msg)}
            Err(e) => {println!("{}",e)}
        }
    }

}