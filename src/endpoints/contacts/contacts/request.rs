use crate::endpoints::contacts::fields::schema::DataType;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Debug)]
pub struct GetListContactFieldsRequest {
    pub account_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct CreateContactFieldRequest {
    #[serde(skip)]
    pub account_id: u64,

    #[builder(setter(into))]
    pub name: String,
    pub data_type: DataType,
    #[builder(setter(into))]
    pub merge_tag: String,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetContactFieldRequest {
    pub account_id: u64,
    pub field_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct UpdateContactFieldRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub field_id: u64,

    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub merge_tag: String,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct DeleteContactFieldRequest {
    pub account_id: u64,
    pub field_id: u64,
}
