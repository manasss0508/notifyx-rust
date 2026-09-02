use serde::Deserialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize,Debug,Clone,FromRow)]
pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub channel: String,
    pub subject: String,
    pub body: String,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub updated_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}