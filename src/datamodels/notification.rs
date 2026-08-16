use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::datamodels::database::notifaction::Notification;

//============= create notification
#[derive(Debug,Deserialize,Validate)]
pub struct CreateNotificationRequest{
    pub channel: String,
    #[validate(email)]
    pub recipient: String,
    pub template: String, //later we will convert this to enum type
    pub variables: HashMap<String,String>,
    #[serde(default="low_value")]
    pub priority: String,
    pub schedule_at: Option<chrono::DateTime<chrono::Utc>>
}
fn low_value() -> String {
    "LOW".to_owned()
}

#[derive(Clone,Debug,Serialize)]
pub struct CreateNotificationResponse{
    pub notification_id: Uuid,
    pub status: String,//later we will convert this to enum type
}

//============= get notification
#[derive(Clone,Debug,Serialize)]
pub struct GetNotificationResponse{
    #[serde(skip_serializing_if="Option::is_none")]
    pub notification: Option<Notification>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>
}
