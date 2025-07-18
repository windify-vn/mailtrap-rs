use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ContactStatus {
    Subscribed,
    Unsubscribed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ContactAction {
    Created,
    Updated,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactDetail {
    pub id: String,
    pub email: String,
    pub fields: serde_json::Value,
    pub list_ids: Vec<u64>,
    pub status: ContactStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(TypedBuilder, Serialize, Default, Debug, Clone)]
pub struct CreateContactParams {
    #[builder(setter(into))]
    pub email: String,
    #[builder(default, setter(strip_option, into))]
    pub fields: Option<serde_json::Value>,
    #[builder(default, setter(strip_option, into))]
    pub list_ids: Option<Vec<u64>>,
}

#[derive(TypedBuilder, Serialize, Default, Debug, Clone)]
pub struct UpdateContactParams {
    #[builder(setter(into))]
    pub email: String,
    #[builder(default, setter(strip_option, into))]
    pub fields: Option<serde_json::Value>,
    #[builder(default, setter(strip_option, into))]
    pub list_ids_included: Option<Vec<u64>>,
    #[builder(default, setter(strip_option, into))]
    pub list_ids_excluded: Option<Vec<u64>>,
    #[builder(default, setter(strip_option, into))]
    pub unsubscribed: Option<bool>,
}
