use axum::{
    extract::{Path,State},
    Json,
};
use crate::service::validation::{notification_validation};

use axum::http::StatusCode;
use serde::Serialize;
use uuid::Uuid;
use crate::configuration::api_state::SharedState;
use crate::datamodels::database::notifaction::Notification;
use crate::datamodels::notification::{
    CreateNotificationRequest,
    CreateNotificationResponse,
    GetNotificationResponse
};
use crate::error::AppError;
use crate::repository::notification::{db_create_notification, db_get_notification};

pub async fn create_notification_handler(State(app_state):State<SharedState>,
                                         Json(payload): Json<CreateNotificationRequest>)
    -> Result<Json<CreateNotificationResponse>,AppError>{
    tracing::info!("creating notification, notif : {:?}",&payload);
    
    // validation
    notification_validation(&payload)?;
    tracing::info!("notification validation success");

    //generate notification id
    let uuid:Uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string();
    tracing::info!("notification id generated");

    // save to database
    let created_notif = db_create_notification(
        &(*app_state).db_pool,
        uuid,
        &payload,
        "PENDING".to_owned(),
    ).await?;
    tracing::info!("notification saved to database");


    // publish message to rbmq
    (*app_state).rbmq_conn.publish(created_notif.id,&created_notif.channel)
        .await?;
    tracing::info!("notification pushed to queue");

    // send response
    Ok(Json(CreateNotificationResponse{
        notification_id: created_notif.id,
        status: created_notif.status,
    }))

}

pub async fn get_notification_handler(Path(notification_id):Path<Uuid>,
                                      State(app_state):State<SharedState>)
    -> Result<Json<GetNotificationResponse>,AppError> {
    tracing::info!("finding notification");

    // get notification from database
    let res = db_get_notification(&(*app_state).db_pool,notification_id)
        .await?; // if notification not exits is handled already
    tracing::info!("notification founded");


    // if notification exits
    Ok(Json(GetNotificationResponse{
        notification: Some(res),
        message: None   ,
    }))
}