use crate::framework::response::JsonResult;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct DeleteProjectResponse {
    pub id: u64,
}

impl JsonResult for DeleteProjectResponse {}
