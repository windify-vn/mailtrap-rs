use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Debug, PartialEq, Eq, Clone)]
pub struct GetAccountBillingRequest {
    pub account_id: u64,
}
