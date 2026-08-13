use crate::datamodels::rbmq::notifation::NotifMsg;
use crate::error::AppError;

// used to deserialize message to "NotifMsg" struct
pub fn deserialize_message(msg_bytes: &Vec<u8>) -> Result<NotifMsg,AppError> {
    let msg:NotifMsg = serde_json::from_slice(msg_bytes).map_err(|e| {
        AppError::Deserialization(e)
    })?;
    Ok(msg)
}