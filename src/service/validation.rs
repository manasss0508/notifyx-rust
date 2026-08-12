use crate::datamodels::notification::CreateNotificationRequest;
use axum::{
    http::StatusCode,
};
use validator::{Validate, ValidationError};
use crate::error::AppError;
use crate::service::error::ValidationErr;

//validate the entire payload
pub fn notification_validation(payload: &CreateNotificationRequest) -> Result<(),AppError> {

    // validate channel and recipient
    validate_channel_and_recipient(payload)?;

    //validate template
    validate_template(&payload.template)?;

    //validation
    Ok(())
}


// validate channel and recipient of the payload
fn validate_channel_and_recipient(payload: &CreateNotificationRequest) -> Result<(),AppError> {
    match payload.channel.as_str() {
        "MAIL" => {
            return validate_recipient_mail(payload);
        },
        _ => {
            tracing::error!("validation: channel not supported");
            return Err(AppError::Validation("provided notification channel is not supported".to_string()));
        },
    };
}

// validate mail of the recipient
fn validate_recipient_mail(payload: &CreateNotificationRequest) -> Result<(),AppError> {
    payload.validate().map_err(|_|{
        tracing::error!("validation: not a valid recipient");
        AppError::Validation("not a valid recipient".to_string())
    })
}

// validate template - check if api support the template
fn validate_template(t: &String) -> Result<(),AppError> {
    match t.as_str()  {
        "WELCOME" => {
            return Ok(())
        },
        _ => {
            tracing::error!("validation: template not available");
            return Err(AppError::Validation("provided template is not available".to_string()))
        }
    }
}
    