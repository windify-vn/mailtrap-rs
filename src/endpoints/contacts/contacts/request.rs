use crate::endpoints::contacts::contacts::schema::{CreateContactParams, UpdateContactParams};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct CreateContactRequest {
    #[serde(skip)]
    pub account_id: u64,

    pub contact: CreateContactParams,
}

#[derive(TypedBuilder, Serialize, Debug)]
pub struct GetContactRequest {
    pub account_id: u64,
    pub contact_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct UpdateContactRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub contact_id: u64,

    pub contact: UpdateContactParams,
}

#[derive(TypedBuilder, Serialize, Debug)]
pub struct DeleteContactRequest {
    pub account_id: u64,
    pub contact_id: u64,
}
