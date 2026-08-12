use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::chrono,
};
use uuid::Uuid;

#[derive(Clone, Debug,Deserialize,FromRow,Serialize)]
pub struct Notification {
    pub id: Uuid,
    pub channel: String,
    pub recipient: String,
    pub template: String,
    pub name: String,
    pub status: String, // "PENDING", "PROCESSING", "SENT", "FAILED", "RETRYING"
    pub priority: String, // "LOW", "MEDIUM", "HIGH"
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub retry_count: i32,
    pub max_retry: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub failure_reason: Option<String>,
}
