use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAccountsResponse {
    pub id: u64,
    pub name: String,
    pub access_levels: Vec<u32>,
}

impl JsonResult for GetAccountsResponse {}
impl JsonResult for Vec<GetAccountsResponse> {}
