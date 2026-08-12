use notifyx_rust::configuration::config;
use notifyx_rust::queue::consumer::create_and_consume_queue;
use notifyx_rust::worker::mail::process_each_message_mail;

#[tokio::main(flavor="multi_thread", worker_threads=3)]
async fn main() {
    // loading all configuration
    let app_state = config::load_for_worker().await;

    // create and consume queue
    let queue_name = "notfication.queue.mail".to_string();
    const worker_type:&str = "MAIL";
    let mut consumer = match  create_and_consume_queue(
        &(*app_state).rbmq_conn,
        queue_name,
        worker_type
    ).await {
        Ok(consumer) => consumer,
        Err(e)  => panic!("{}",e)
    };

    // process each message
    if let Err(e) = process_each_message_mail(app_state,consumer).await{
        println!("{}",e)
    }else {
        println!("consumer stream ended : queue deleted or stream cancelled")
    }

}