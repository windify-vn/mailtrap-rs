use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod accounts;
pub mod addresses;
pub mod billing;
pub mod permissions;

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum_macros::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ResourceType {
    #[default]
    Account,
    Billing,
    Project,
    Inbox,
    SendingDomain,
    EmailCampaignPermissionScope,
    EmailTemplatePermissionScope,
}

#[derive(
    Serialize_repr, Deserialize_repr, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
#[repr(u32)]
pub enum AccessLevel {
    Owner = 1000,
    Admin = 100,
    ViewerPlus = 50,
    #[default]
    Viewer = 10,
    Indeterminate = 1,
}
