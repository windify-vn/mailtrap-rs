use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactField {
    pub id: u64,
    pub name: String,
    pub data_type: DataType,
    pub merge_tag: String,
}

impl JsonResult for ContactField {}
impl JsonResult for Vec<ContactField> {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Text,
    Integer,
    Float,
    Boolean,
    Date,
}
