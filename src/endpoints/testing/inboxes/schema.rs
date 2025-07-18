use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InboxStatus {
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InboxDetail {
    pub id: u64,
    pub name: String,
    pub username: String,
    pub password: Option<String>,
    pub max_size: u64,
    pub status: InboxStatus,
    pub email_username: String,
    pub email_username_enabled: bool,
    pub sent_messages_count: u64,
    pub forwarded_messages_count: u64,
    pub used: bool,
    pub forward_from_email_address: String,
    pub project_id: u64,
    pub domain: String,
    pub pop3_domain: String,
    pub api_domain: String,
    pub email_domain: String,
    pub emails_count: u64,
    pub emails_unread_count: u64,
    pub last_message_sent_at: Option<String>,
    pub smtp_ports: Vec<u16>,
    pub pop3_ports: Vec<u16>,
    pub max_message_size: u64,
    pub permissions: InboxPermission,
}

impl ApiResult for InboxDetail {}
impl ApiResult for Vec<InboxDetail> {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InboxPermission {
    pub can_read: bool,
    pub can_update: bool,
    pub can_destroy: bool,
    pub can_leave: bool,
}

#[derive(TypedBuilder, Debug, Serialize, Deserialize, Default, Clone)]
pub struct CreateInboxPrams {
    #[builder(setter(into))]
    pub name: String,
}

#[derive(TypedBuilder, Debug, Serialize, Deserialize, Default, Clone)]
pub struct UpdateInboxPrams {
    #[builder(setter(strip_option, into))]
    pub name: Option<String>,

    #[builder(setter(strip_option, into))]
    pub email_username: Option<String>,
}
