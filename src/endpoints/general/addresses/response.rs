use crate::endpoints::general::addresses::schema::{
    AccessPermissions, AccessResource, AccessSpecifier, SpecifierType,
};
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetListAccessesResponse {
    pub id: u64,
    pub specifier_type: SpecifierType,
    pub specifier: AccessSpecifier,
    pub resources: Vec<AccessResource>,
    pub permission: AccessPermissions,
}

impl ApiResult for GetListAccessesResponse {}
impl ApiResult for Vec<GetListAccessesResponse> {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteAccessResponse {
    pub id: u64,
}

impl ApiResult for DeleteAccessResponse {}
