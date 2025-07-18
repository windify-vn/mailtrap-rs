use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttachmentType {
    Inline,
    #[default]
    Attachment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachmentDetail {
    pub id: u64,
    pub message_id: u64,
    pub filename: String,
    pub content_type: String,
    pub content_id: Option<String>,
    pub transfer_encoding: Option<String>,
    pub attachment_size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub attachment_human_size: String,
    pub download_path: String,
}

impl JsonResult for AttachmentDetail {}
impl JsonResult for Vec<AttachmentDetail> {}
