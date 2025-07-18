use crate::endpoints::sending::domains::SendingDomainDetail;
use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListDomainResponse {
    pub data: Vec<SendingDomainDetail>,
}

impl JsonResult for ListDomainResponse {}
