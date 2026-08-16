use std::collections::HashMap;
use crate::datamodels::notification::CreateNotificationRequest;
use axum::{
    http::StatusCode,
};
use validator::{Validate, ValidationError};
use crate::error::AppError;
use crate::service::error::ValidationErr;
use crate::template_engine::engine::json_string_to_template_variables;

//validate the entire payload
pub fn notification_validation(payload: &CreateNotificationRequest) -> Result<(),AppError> {

    // validate channel and recipient
    validate_channel_and_recipient(payload)?;

    //validate template
    validate_template(&payload.template)?;

    // validate variables
    validate_variables(&payload.variables,&payload.template)?;

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
        "welcome" => {
            return Ok(())
        },
        "otp" => {
            return Ok(())
        },
        "password_reset" => {
            return Ok(())
        },
        "email_verification" => {
            return Ok(())
        },
        "login_alert" => {
            return Ok(())
        },
        "order_confirmation" => {
            return Ok(())
        },
        "order_shipped" => {
            return Ok(())
        },
        "payment_success" => {
            return Ok(())
        },
        "payment_failed" => {
            return Ok(())
        },
        "subscription_renewal" => {
            return Ok(())
        },
        _ => {
            tracing::error!("validation: template not available");
            return Err(AppError::Validation("provided template is not available".to_string()))
        }
    }
}

fn validate_variables(json_obj: &HashMap<String,String>,template_name:&String) -> Result<(),AppError> {
    let json_string = serde_json::to_string(json_obj).map_err(|e|{
        tracing::error!("validation: json object is not valid");
        AppError::Validation("validation: json object is not valid".to_string())
    })?;

    if let Ok(_) = json_string_to_template_variables(template_name,&json_string) {
        Ok(())
    }else {
        tracing::error!("validation: variables are insufficient");
        Err(AppError::Validation("validation: variables are insufficient".to_string()))
    }
}
    