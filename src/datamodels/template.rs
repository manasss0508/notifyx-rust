use serde::Deserialize;

pub enum TemplateVariables {
    Welcome(WelcomeVariables),
    Otp(OtpVariables),
    PasswordReset(PasswordResetVariables),
    EmailVerification(EmailVerificationVariables),
    LoginAlert(LoginAlertVariables),
    OrderConfirmation(OrderConfirmationVariables),
    OrderShipped(OrderShippedVariables),
    PaymentSuccess(PaymentSuccessVariables),
    PaymentFailed(PaymentFailedVariables),
    SubscriptionRenewal(SubscriptionRenewalVariables)
}

// name
#[derive(Debug,Deserialize)]
pub struct WelcomeVariables {
    pub name: String,
}
// name, otp, expiry_minutes
#[derive(Debug,Deserialize)]
pub struct OtpVariables{
    pub name: String,
    pub otp: String,
    pub expiry_minutes: String,
}
// name, reset_link, expiry_minutes
#[derive(Debug,Deserialize)]
pub struct PasswordResetVariables{
    pub name: String,
    pub reset_link: String,
    pub expiry_minutes: String,
}
// name, verification_link
#[derive(Debug,Deserialize)]
pub struct EmailVerificationVariables{
    pub name: String,
    pub verification_link: String
}
// name, login_time, location, device
#[derive(Debug,Deserialize)]
pub struct LoginAlertVariables{
    pub name: String,
    pub login_time: String,
    pub location: String,
    pub device: String
}
// order_id, name, amount, currency,
#[derive(Debug,Deserialize)]
pub struct OrderConfirmationVariables{
    pub order_id: String,
    pub name: String,
    pub amount: String,
    pub currency: String
}
// name, order_id, tracking_number, estimated_delivery
#[derive(Debug,Deserialize)]
pub struct OrderShippedVariables{
    pub name: String,
    pub order_id: String,
    pub tracking_number: String,
}
// name, transaction_id, amount, currency
#[derive(Debug,Deserialize)]
pub struct PaymentSuccessVariables{
    pub name: String,
    pub transaction_id: String,
    pub amount: String,
    pub currency: String
}
// name, amount, currency , reason
#[derive(Debug,Deserialize)]
pub struct PaymentFailedVariables{
    pub name: String,
    pub amount: String,
    pub currency: String,
    pub reason: String,
}
// name, plan_name, renewal_date, net_billing_date
#[derive(Debug,Deserialize)]
pub struct SubscriptionRenewalVariables{
    pub name: String,
    pub plan_name: String,
    pub renewal_date: String,
    pub net_billing_date: String,
}