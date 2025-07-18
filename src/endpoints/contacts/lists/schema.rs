use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactList {
    pub id: u64,
    pub name: String,
}

impl JsonResult for ContactList {}
impl JsonResult for Vec<ContactList> {}
