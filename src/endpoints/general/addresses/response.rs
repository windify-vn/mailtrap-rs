use crate::endpoints::general::addresses::schema::{
    AccessPermissions, AccessResource, AccessSpecifier, SpecifierType,
};
use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetListAccessesResponse {
    pub id: u64,
    pub specifier_type: SpecifierType,
    pub specifier: AccessSpecifier,
    pub resources: Vec<AccessResource>,
    pub permission: AccessPermissions,
}

impl JsonResult for GetListAccessesResponse {}
impl JsonResult for Vec<GetListAccessesResponse> {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteAccessResponse {
    pub id: u64,
}

impl JsonResult for DeleteAccessResponse {}
