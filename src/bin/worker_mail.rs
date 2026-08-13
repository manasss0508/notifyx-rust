use notifyx_rust::configuration::config;
use notifyx_rust::queue::consumer::create_and_consume_queue;
use notifyx_rust::worker::mail::process_each_message_mail;

#[tokio::main(flavor="multi_thread", worker_threads=3)]
async fn main() {
    const WORKER_TYPE:&str = "MAIL";

    // loading all configuration
    let app_state = config::load_for_worker().await;

    // create and consume queue
    let queue_name = std::env::var("RABBITMQ_QUEUE_MAIL_NAME")
        .expect("env variable RABBITMQ_QUEUE_MAIL_NAME is not set");

    // creating queue and consuming it
    let mut consumer = match  create_and_consume_queue(
        &(*app_state).rbmq_conn,
        queue_name,
        WORKER_TYPE
    ).await {
        Ok(consumer) => consumer,
        Err(e)  => panic!("{}",e)
    };

    // process each message
    println!("worker started");
    process_each_message_mail(app_state,consumer).await;
}