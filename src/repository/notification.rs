use axum::http::StatusCode;
use sqlx::{query, PgPool, query_as};
use uuid::Uuid;
use std::convert::From;
use crate::datamodels::{
    database::notifaction::{Notification,NotificationRetryCount},
    notification::CreateNotificationRequest,
};
use crate::error::AppError;
use crate::error::AppError::DbNotFound;

// to create notification in database
pub async fn db_create_notification(conn: &PgPool, notif_id: Uuid, notif: &CreateNotificationRequest, notif_status: String)
                                -> Result<Notification,AppError> {
    let variables = serde_json::to_string(&notif.variables)?;

    // without schedule_at
    sqlx::query_as!(
        Notification,
        r#"
INSERT INTO notifications(id,channel,recipient,template,variables,priority,status)
VALUES ($1,$2,$3,$4,$5,$6,$7)
RETURNING *;
"#,
        notif_id,
        &notif.channel,
        &notif.recipient,
        &notif.template,
        variables.as_str(),
        &notif.priority,
        notif_status
    ).fetch_one(conn)
        .await
        .map_err(|e|{
            tracing::error!("database : {:?}", e);
            AppError::Database(e)
        })
}

// to get notification by id from database
pub async fn db_get_notification(conn: &PgPool,notif_id:Uuid) -> Result<Notification,AppError>{
    let notif = sqlx::query_as!(
        Notification,
        r#"
SELECT *
FROM notifications
WHERE id=$1;
"#,
        notif_id
    ).fetch_optional(conn)
        .await
        .map_err(|e|{
        tracing::error!("database : {:?}", e);
        AppError::Database(e)
    })?;

    // if notfication exits in database
    if let Some(notif) = notif {
        return Ok(notif)
    }

    // if notfication not extis in database
    tracing::error!("database: notification not available in database ", );
    Err(AppError::DbNotFound)
}

pub async fn db_update_notification_status(conn: &PgPool,notif_id:Uuid, status: &str) -> Result<(),AppError> {
    let res = sqlx::query!(
        r#"
UPDATE notifications
SET status=$2
WHERE id=$1
RETURNING *;
"#,
        notif_id,
        status
    ).fetch_optional(conn)
        .await
        .map_err(|e|{
        tracing::error!("database : {:?}", e);
        AppError::Database(e)
    });

    match res {
        // query execute
        Ok(row) => {
            match row {
                // record updated
                Some(_) => {
                    Ok(())
                },
                // record not available
                None => {
                    Err(AppError::DbNotFound)
                }
            }
        },
        // query failed
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn db_update_notification_retry_count(conn: &PgPool,notif_id:Uuid, retry_count: i32) -> Result<(),AppError> {
    let res = query!(
        r#"
UPDATE notifications
SET retry_count=$2
WHERE id=$1
"#,
        notif_id,
        retry_count,
    ).fetch_optional(conn)
        .await
    .map_err(|e|{
        tracing::error!("database : {:?}", e);
        AppError::Database(e)
    });

    match res {
        // query execute
        Ok(row) => {
            match row {
                // record updated
                Some(_) => {
                    Ok(())
                },
                // record not available
                None => {
                    Err(AppError::DbNotFound)
                }
            }
        },
        // query failed
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn db_get_notification_retry_count(conn: &PgPool,notif_id:Uuid) -> Result<i32,AppError> {
    let result = query_as!(
        NotificationRetryCount,
        r#"
SELECT retry_count
FROM notifications
WHERE id=$1;
"#,
        notif_id
    ).fetch_optional(conn)
        .await
        .map_err(|e|{
            tracing::error!("database : {:?}", e);
            AppError::Database(e)
        });

    match result {
        // query execute
        Ok(record) => {
            match record {
                // record exist
                Some(record) => {
                    Ok(record.retry_count)
                },
                // record not available
                None => {
                    Err(AppError::DbNotFound)
                }
            }
        },
        // query failed
        Err(e) => {
            Err(e)
        }
    }
}