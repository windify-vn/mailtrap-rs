use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailTemplate {
    pub id: u64,
    pub uuid: uuid::Uuid,
    pub name: String,
    pub subject: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl JsonResult for EmailTemplate {}
impl JsonResult for Vec<EmailTemplate> {}

#[derive(TypedBuilder, Debug, Serialize, Deserialize, Default, Clone)]
pub struct EmailTemplatesParams {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub category: String,
    #[builder(setter(into))]
    pub subject: String,

    #[builder(setter(strip_option, into))]
    pub body_text: Option<String>,
    #[builder(setter(strip_option, into))]
    pub body_html: Option<String>,
}
