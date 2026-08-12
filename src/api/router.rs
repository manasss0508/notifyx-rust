use axum::{
    Router,
    routing::{get,post},
};

use tower_http::trace::{TraceLayer,DefaultOnRequest,DefaultOnResponse};

use crate::api::handlers::{create_notification_handler,get_notification_handler};
use crate::configuration::api_state::SharedState;
use tracing::{Level};

pub fn api_router(state: SharedState) -> Router {
    Router::new()
        .route("/notification",post(create_notification_handler))
        .route("/notification/:notification_id",get(get_notification_handler))
        .layer(TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)))
        .with_state(state)
}
