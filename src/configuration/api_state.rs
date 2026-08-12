use std::sync::Arc;
use sqlx::PgPool;
use crate::queue::producer::QueueConn;

pub struct AppState {
    pub db_pool: PgPool, // database connection
    pub rbmq_conn: QueueConn, // connection to rabbitmq
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new(db_pool: PgPool,rbmq_conn:QueueConn) -> SharedState {
        Arc::new(AppState {
            db_pool,
            rbmq_conn
        })
    }
}

