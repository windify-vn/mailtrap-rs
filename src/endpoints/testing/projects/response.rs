use crate::framework::response::ApiResult;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct DeleteProjectResponse {
    pub id: u64,
}

impl ApiResult for DeleteProjectResponse {}
