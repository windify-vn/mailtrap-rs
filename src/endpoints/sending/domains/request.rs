use crate::endpoints::sending::domains::SendingDomain;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct CreateDomainRequest {
    #[serde(skip)]
    pub account_id: u64,

    pub sending_domain: SendingDomain,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct ListDomainRequest {
    pub account_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetDomainRequest {
    pub account_id: u64,
    pub sending_domain_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct DeleteDomainRequest {
    pub account_id: u64,
    pub sending_domain_id: u64,
}

#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct SendSetupDomainInstructionRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub sending_domain_id: u64,

    #[builder(setter(into))]
    pub email: String,
}
