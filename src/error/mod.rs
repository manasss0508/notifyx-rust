use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::Serialize;
use crate::service::error::ValidationErr;

// error response that will sent from handler
#[derive(Debug,Serialize)]
struct ErrorResponse{
    error: ErrorBody
}

// error body
#[derive(Debug,Serialize)]
struct  ErrorBody{
    code: String,
    message: String,
}

// AppError - a error type which represent all error that can occured in this app
// it impl Error , so it can integrate in rust error ecosystem
// it impl into_response, so it can used as response type
// in handler it can be used as response type
#[derive(Debug,thiserror::Error)]
pub enum AppError {
    // error related to validation
    #[error("validation failed : {0}")]
    Validation(String),

    // error related to database operation
    #[error("database operation failed")]
    Database(#[from] sqlx::Error), //()

    // if notification not found in database
    #[error("notification not found database")]
    DbNotFound,

    // error related to rabbitmq
    #[error("message queue operation failed")]
    Queue(#[from] lapin::Error),

    #[error("routing key not exist")]
    RoutingKeyNotExist,

    //
    #[error("serialization failed")]
    Serialization(#[from] serde_json::Error),

    #[error("deserialization failed")]
    Deserialization(serde_json::Error),

    #[error("serde_json::value conversion to string failed")]
    SerdeValueToJsonStringFailed

}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let(sc,code,message) =  match self {
            AppError::Validation(e) => {
                (StatusCode::BAD_REQUEST,
                 "VALIDATION_ERROR",
                 e
                )
            },
            AppError::DbNotFound => {
                (StatusCode::NOT_FOUND,"NOT_FOUND",
                 "notIfication does not exists".to_string())
            },

            _ => (StatusCode::INTERNAL_SERVER_ERROR,"INTERNAL_SERVER_ERROR",
            "error occurred in api".to_string())
        };

        // converting to response
        (sc,Json(ErrorResponse{
            error: ErrorBody {
                code: code.to_string(),
                message: message,
            }
        })).into_response()
    }
}