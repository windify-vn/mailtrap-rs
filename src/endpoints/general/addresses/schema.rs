use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, strum_macros::AsRefStr,
)]
#[serde(rename_all = "PascalCase")]
#[strum(serialize_all = "PascalCase")]
pub enum SpecifierType {
    User,
    Invite,
    ApiToken,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, strum_macros::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ResourceType {
    Account,
    Billing,
    Project,
    Inbox,
    SendingDomain,
    EmailCampaignPermissionScope,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccessLevel {
    Owner = 1000,
    Admin = 100,
    ViewerPlus = 50,
    Viewer = 10,
    Indeterminate = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessSpecifier {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub two_factor_authentication_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessResource {
    pub resource_id: u64,
    pub resource_type: ResourceType,
    pub access_level: AccessLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessPermissions {
    pub can_read: bool,
    pub can_update: bool,
    pub can_destroy: bool,
    pub can_leave: bool,
}
