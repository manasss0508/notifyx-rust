use std::sync::Arc;
use sqlx::PgPool;
use crate::queue::producer::QueueConn;
use crate::service::email::EmailService;
use crate::template_engine::cache::TemplateCache;

pub struct AppState {
    pub db_pool: PgPool, // database connection
    pub rbmq_conn: QueueConn, // connection to rabbitmq
    pub template_cache: TemplateCache, // template cache
    pub email_service: EmailService, // mail sender
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub fn new(db_pool: PgPool,
               rbmq_conn:QueueConn,
               template_cache: TemplateCache,
               email_service: EmailService) -> SharedState {
        Arc::new(AppState {
            db_pool,
            rbmq_conn,
            template_cache,
            email_service
        })
    }
}

