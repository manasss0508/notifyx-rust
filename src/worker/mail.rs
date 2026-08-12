use lapin::Consumer;
use crate::configuration::api_state::SharedState;
use crate::error::AppError;
use futures_util::StreamExt;

pub async fn process_each_message_mail(state: SharedState,mut consumer: Consumer) -> Result<(),AppError> {

    // loop will end when we get None
    // we get none when consumer stream ends
    while let Some(msg) = consumer.next().await {
        let msg = msg?;
        println!("message received: {:?}", msg)
    }

    Ok(())
}