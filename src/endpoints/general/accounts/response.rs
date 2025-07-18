use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAccountsResponse {
    pub id: u64,
    pub name: String,
    pub access_levels: Vec<u32>,
}

impl ApiResult for GetAccountsResponse {}
impl ApiResult for Vec<GetAccountsResponse> {}
