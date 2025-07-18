use crate::endpoints::general::permissions::PermissionResource;
use crate::endpoints::general::{AccessLevel, ResourceType};
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateTokenPermissionsResponse {
    pub message: String,
}

impl ApiResult for UpdateTokenPermissionsResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPermissionResourceResponse {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    pub access_level: AccessLevel,

    pub resources: Vec<PermissionResource>,
}

impl ApiResult for GetPermissionResourceResponse {}
impl ApiResult for Vec<GetPermissionResourceResponse> {}
