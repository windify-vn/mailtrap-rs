use crate::endpoints::general::{AccessLevel, ResourceType};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TokenPermission {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub access_level: AccessLevel,

    #[builder(default, setter(strip_option, into))]
    #[serde(rename = "_destroy")]
    pub destroy: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionResource {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    pub access_level: AccessLevel,
}
