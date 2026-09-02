use std::collections::HashMap;
use sqlx::PgPool;
use tokio::sync::RwLock;
use crate::datamodels::database::template::Template;
use crate::error::AppError;
use crate::repository::template::db_get_template;

pub struct TemplateCache{
    mail: RwLock<HashMap<String,Template>>,
    sms: RwLock<HashMap<String,Template>>
}

impl TemplateCache {
    pub fn new() -> Self {
        TemplateCache{
            mail: RwLock::new(HashMap::new()),
            sms: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_template_mail(&self,db_pool:&PgPool,template_name: &String, channel_type: &String)
        -> Result<Template,AppError> {
            // check if template exist in cache
            let read = (&self.mail).read().await;
            let template = (*read).get(template_name);

            // if template exits
            if let Some(template) = template {
                return Ok(template.clone())
            }

        drop(read);

        // template not exits

        // get template from database
        let template = db_get_template(db_pool,channel_type,template_name).await?;

        // getting write lock
        let mut write = self.mail.write().await;
        // adding template to map
        (*write).insert(template.name.clone(),template.clone());

        Ok(template)

    }

    pub async fn get_template_sms(&self,db_pool:&PgPool,template_name: &String, channel_type: &String)
        -> Result<Template,AppError> {
        // check if template exist in cache
        let read = (&self.sms).read().await;
        let template = (*read).get(template_name);


        // if template exits
        if let Some(template) = template {
            Ok(template.clone())
        }else { // template not exits

            // get template from database
            let template = db_get_template(db_pool,channel_type,template_name).await?;

            // getting write lock
            let mut write = self.mail.write().await;
            // adding template to map
            (*write).insert(template.name.clone(),template.clone());

            Ok(template)
        }
    }

}