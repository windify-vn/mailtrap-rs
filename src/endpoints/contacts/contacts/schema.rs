use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactField {
    pub id: u64,
    pub name: String,
    pub data_type: DataType,
    pub merge_tag: String,
}

impl ApiResult for ContactField {}
impl ApiResult for Vec<ContactField> {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Text,
    Integer,
    Float,
    Boolean,
    Date,
}
