use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Debug)]
pub struct GetListContactListsRequest {
    pub account_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct CreateContactListsRequest {
    #[serde(skip)]
    pub account_id: u64,

    #[builder(setter(into))]
    pub name: String,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetContactListsRequest {
    pub account_id: u64,
    pub list_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct UpdateContactListsRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub list_id: u64,

    #[builder(setter(into))]
    pub name: String,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct DeleteContactListsRequest {
    pub account_id: u64,
    pub list_id: u64,
}
