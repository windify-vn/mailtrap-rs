use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, strum_macros::AsRefStr,
)]
pub enum SuppressionType {
    #[serde(rename = "hard bounce")]
    HardBounce,
    #[serde(rename = "spam complaint")]
    SpamComplaint,
    #[serde(rename = "unsubscription")]
    Unsubscription,
    #[serde(rename = "manual import")]
    ManualImport,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, strum_macros::AsRefStr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SendingStream {
    Any,
    Transactional,
    Bulk,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuppressionDetail {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub sending_stream: SendingStream,
    pub domain_name: Option<String>,
    pub message_bounce_category: Option<String>,
    pub message_category: Option<String>,
    pub message_client_ip: Option<String>,
    pub message_created_at: Option<String>,
    pub message_esp_response: Option<String>,
    pub message_esp_server_type: Option<String>,
    pub message_outgoing_ip: Option<String>,
    pub message_recipient_mx_name: Option<String>,
    pub message_sender_email: Option<String>,
    pub message_subject: Option<String>,
}

impl JsonResult for SuppressionDetail {}
impl JsonResult for Vec<SuppressionDetail> {}
