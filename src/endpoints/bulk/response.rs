use crate::endpoints::sending::emails::BatchEmailResponse;
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendBulkEmailResponse {
    pub success: bool,
    #[serde(default)]
    pub messages_ids: Vec<String>,
}

impl ApiResult for SendBulkEmailResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchSendBulkEmailResponse {
    pub success: bool,
    #[serde(default)]
    pub responses: Vec<BatchEmailResponse>,
    #[serde(default)]
    pub errors: Vec<String>,
}

impl ApiResult for BatchSendBulkEmailResponse {}
