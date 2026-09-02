use sqlx::PgPool;
use crate::datamodels::database::notifaction::Notification;
use crate::datamodels::database::template::Template;
use crate::error::AppError;

pub async fn db_get_template(db_pool: &PgPool, channel_type: &String, template_name: &String) -> Result<Template,AppError> {
    return sqlx::query_as!(Template,
        r#"
SELECT *
FROM templates
WHERE channel=$1 AND name=$2
"#,
        channel_type,
        template_name,
    ).fetch_one(db_pool)
        .await
        .map_err(|e|{
            tracing::error!("database : {:?}", e);
            AppError::Database(e)
        })

}