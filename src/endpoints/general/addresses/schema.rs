use crate::endpoints::general::{AccessLevel, ResourceType};
use serde::{Deserialize, Serialize};

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
