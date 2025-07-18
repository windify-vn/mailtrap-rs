use crate::endpoints::contacts::contacts::schema::{ContactAction, ContactDetail};
use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateContactResponse {
    pub data: ContactDetail,
}

impl JsonResult for CreateContactResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetContactResponse {
    pub data: ContactDetail,
}
impl JsonResult for GetContactResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateContactResponse {
    pub action: ContactAction,
    pub data: ContactDetail,
}
impl JsonResult for UpdateContactResponse {}
