use crate::endpoints::sending::emails::BatchEmailResponse;
use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendEmailResponse {
    pub success: bool,
    #[serde(default)]
    pub messages_ids: Vec<String>,
}

impl JsonResult for SendEmailResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchSendEmailResponse {
    pub success: bool,
    #[serde(default)]
    pub responses: Vec<BatchEmailResponse>,
    #[serde(default)]
    pub errors: Vec<String>,
}

impl JsonResult for BatchSendEmailResponse {}
