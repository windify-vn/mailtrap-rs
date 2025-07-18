use crate::endpoints::sending::domains::{
    ComplianceStatus, DnsRecord, RecordPermission, SendingDomainDetail,
};
use crate::endpoints::sending::emails::BatchEmailResponse;
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListDomainResponse {
    pub data: Vec<SendingDomainDetail>,
}

impl ApiResult for ListDomainResponse {}
