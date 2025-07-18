use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct ListSuppressionsRequest {
    #[serde(skip)]
    pub account_id: u64,

    #[builder(setter(into))]
    pub email: Option<String>,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct DeleteSuppressionsRequest {
    pub account_id: u64,
    pub suppression_id: uuid::Uuid,
}
