use crate::endpoints::general::permissions::TokenPermission;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct UpdateTokenPermissionsRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub account_access_id: u64,

    #[builder(default, setter(into))]
    pub permissions: Vec<TokenPermission>,
}

#[derive(TypedBuilder, Debug, PartialEq, Eq, Clone)]
pub struct GetPermissionResourcesRequest {
    pub account_id: u64,
}
