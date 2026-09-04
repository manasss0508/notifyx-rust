use tracing_subscriber::prelude::*;
use crate::repository::create_connection::create_db_pool;
use crate::configuration::api_state::{AppState,SharedState};
use crate::queue;
use crate::template_engine::cache::TemplateCache;
use crate::service::email::EmailService;

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

    // email service
    let host = std::env::var("SMTP_HOST").expect("ENV variable SMTP_HOST not available");
    let username = std::env::var("SMTP_USERNAME").expect("ENV variable SMTP_USERNAME not available");
    let password =  std::env::var("SMTP_PASSWORD").expect("ENV variable SMTP_PASSWORD not available");
    let from =  std::env::var("SMTP_FROM").expect("ENV variable SMTP_FROM not available");
    let email_service = EmailService::new(username,password,from,host);

    // application state
    let app_state = AppState::new(db,rbmq,cache,email_service);

    app_state
}
