use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug, Serialize, Deserialize, Clone)]
pub struct ContactImport {
    #[builder(setter(into))]
    pub email: String,
    pub fields: serde_json::Value,
    pub list_ids_included: Vec<u64>,
    pub list_ids_excluded: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactImportDetail {
    pub id: u64,
    pub status: ImportStatus,
    pub created_contacts_count: Option<i64>,
    pub updated_contacts_count: Option<i64>,
    pub contacts_over_limit_count: Option<i64>,
}

impl JsonResult for ContactImportDetail {}
impl JsonResult for Vec<ContactImportDetail> {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ImportStatus {
    Created,
    Started,
    Finished,
    Failed,
}
