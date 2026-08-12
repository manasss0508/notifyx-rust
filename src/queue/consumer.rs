use crate::error::AppError;
use crate::queue::producer::QueueConn;

pub async fn create_and_consume_queue(
    q : &QueueConn,
    queue_name: String,
    worker_type : &str,
) -> Result<lapin::Consumer,AppError>{

    // create and bind queue, queue will created and bind to notification exchange
    let queue = q.create_queue_and_bind(queue_name,worker_type).await?;

    // consume the queue
    let consumer = q.create_consume(queue.name().to_string())
        .await?;

    Ok(consumer)
}