use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct GetListAccessesRequest {
    #[serde(skip)]
    pub account_id: u64,

    #[builder(default, setter(strip_option, into))]
    pub domain_ids: Option<Vec<String>>,
    #[builder(default, setter(strip_option, into))]
    pub inbox_ids: Option<Vec<String>>,
    #[builder(default, setter(strip_option, into))]
    pub project_ids: Option<Vec<String>>,
}

#[derive(TypedBuilder, Default, Debug, PartialEq, Eq, Clone)]
pub struct DeleteAccessRequest {
    pub account_access_id: u64,
    pub account_id: u64,
}
