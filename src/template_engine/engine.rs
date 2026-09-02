use serde::de::DeserializeOwned;
use crate::error::AppError;
use serde::Deserialize;
use crate::datamodels::template::TemplateVariables;
pub fn json_string_to_template_variables(template_name: &String, variable_json: &String)
    -> Result<TemplateVariables,AppError> {
    match template_name.as_str() {
        "welcome" => {
            return Ok(TemplateVariables::Welcome(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "otp" => {
            return Ok(TemplateVariables::Otp(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "password_reset" => {
            return Ok(TemplateVariables::PasswordReset(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "email_verification" => {
            return Ok(TemplateVariables::EmailVerification(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "login_alert" => {
            return Ok(TemplateVariables::LoginAlert(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "order_confirmation" => {
            return Ok(TemplateVariables::OrderConfirmation(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "order_shipped" => {
            return Ok(TemplateVariables::OrderShipped(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "payment_success" => {
            return Ok(TemplateVariables::PaymentSuccess(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "payment_failed" => {
            return Ok(TemplateVariables::PaymentFailed(serde_json::from_slice(variable_json.as_bytes())?));
        },
        "subscription_renewal" => {
            return Ok(TemplateVariables::SubscriptionRenewal(serde_json::from_slice(variable_json.as_bytes())?));
        },
        _ => Err(AppError::Validation("template not valid".to_string()))
    }
}