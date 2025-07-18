use crate::endpoints::contacts::imports::schema::ContactImport;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Debug)]
pub struct ImportContactsRequest {
    #[serde(skip)]
    pub account_id: u64,

    pub contacts: Vec<ContactImport>,
}

#[derive(TypedBuilder, Debug)]
pub struct GetImportContactRequest {
    pub account_id: u64,
    pub import_id: u64,
}
