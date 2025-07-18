use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct SendingDomain {
    #[builder(setter(into))]
    pub domain_name: String,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, strum_macros::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ComplianceStatus {
    Initial,
    UnverifiedDns,
    Demo,
    MissingCompanyInfo,
    Pending,
    UnderReview,
    NonCompliant,
    Suspended,
    Compliant,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, strum_macros::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DnsStatus {
    Pass,
    Fail,
    Unchecked,
    Missing,
    Softfail,
    NetworkError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsRecord {
    pub key: String,
    pub domain: String,
    pub name: String,
    pub status: DnsStatus,
    #[serde(rename = "type")]
    pub record_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecordPermission {
    pub can_read: bool,
    pub can_update: bool,
    pub can_destroy: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendingDomainDetail {
    pub id: u64,
    pub domain_name: String,
    pub demo: bool,
    pub compliance_status: ComplianceStatus,
    pub dns_verified: bool,
    pub dns_verified_at: Option<String>,
    pub dns_records: Vec<DnsRecord>,
    pub open_tracking_enabled: bool,
    pub click_tracking_enabled: bool,
    pub auto_unsubscribe_link_enabled: bool,
    pub custom_domain_tracking_enabled: bool,
    pub health_alerts_enabled: bool,
    pub critical_alerts_enabled: bool,
    pub alert_recipient_email: Option<String>,
    pub permissions: RecordPermission,
}

impl ApiResult for SendingDomainDetail {}
impl ApiResult for Vec<SendingDomainDetail> {}
