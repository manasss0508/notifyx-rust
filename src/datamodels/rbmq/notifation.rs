use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug,Serialize,Deserialize)]
pub struct NotifMsg {
    pub notification_id: Uuid
}