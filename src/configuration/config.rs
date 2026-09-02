use tracing_subscriber::prelude::*;
use crate::repository::create_connection::create_db_pool;
use crate::configuration::api_state::{AppState,SharedState};
use crate::queue;
use crate::template_engine::cache::TemplateCache;

// loads all basic configuration and initialize application state for API
pub async fn load() -> SharedState {
    // loading environment variable
    dotenvy::dotenv().ok();

    // tracing subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // creating database connection pool
    let db = create_db_pool(10)
        .await;

    // creating rabbitmq connection
    let rbmq = queue::producer::QueueConn::new().await;

    // template cache
    let cache = TemplateCache::new();

    // application state
    let app_state = AppState::new(db,rbmq,cache);

    app_state
}

// loads all basic configuration and initialize application state for worker
pub async fn load_for_worker() -> SharedState {
    // loading environment variable
    dotenvy::dotenv().ok();

    // tracing subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // creating database connection pool
    let db = create_db_pool(10)
        .await;

    // creating rabbitmq connection
    let rbmq = queue::producer::QueueConn::new().await;

    // template cache
    let cache = TemplateCache::new();

    // application state
    let app_state = AppState::new(db,rbmq,cache);
    
    app_state
}