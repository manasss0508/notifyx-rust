
//all errors related to validation
#[derive(Debug,thiserror::Error)]
pub enum ValidationErr{
    #[error("recipient validation failed")]
    InvalidRecipient,

    #[error("unsupported notification channel")]
    InvalidChannel,

    #[error("unsupported template")]
    UnsupportedTemplate
}