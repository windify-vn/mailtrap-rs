use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactList {
    pub id: u64,
    pub name: String,
}

impl ApiResult for ContactList {}
impl ApiResult for Vec<ContactList> {}
