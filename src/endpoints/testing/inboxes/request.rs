use crate::endpoints::testing::inboxes::{CreateInboxParams, UpdateInboxParams};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Debug)]
pub struct GetInboxRequest {
    pub account_id: u64,
    pub inbox_id: u64,
}

#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct CreateInboxRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub project_id: u64,

    pub inbox: CreateInboxParams,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct DeleteInboxRequest {
    pub account_id: u64,
    pub inbox_id: u64,
}

#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct UpdateInboxRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub inbox_id: u64,

    pub inbox: UpdateInboxParams,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct CleanInboxRequest {
    pub account_id: u64,
    pub inbox_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct MarkAsReadInboxRequest {
    pub account_id: u64,
    pub inbox_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct ResetCredentialInboxRequest {
    pub account_id: u64,
    pub inbox_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct ToggleEmailInboxRequest {
    pub account_id: u64,
    pub inbox_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct ResetEmailInboxRequest {
    pub account_id: u64,
    pub inbox_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetListInboxRequest {
    pub account_id: u64,
}
