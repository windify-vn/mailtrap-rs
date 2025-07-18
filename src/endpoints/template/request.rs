use crate::endpoints::template::EmailTemplatesParams;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Debug)]
pub struct GetListEmailTemplatesRequest {
    pub account_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct CreateEmailTemplatesRequest {
    #[serde(skip)]
    pub account_id: u64,

    pub email_template: EmailTemplatesParams,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetEmailTemplatesRequest {
    pub account_id: u64,
    pub email_template_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct UpdateEmailTemplatesRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub email_template_id: u64,

    pub email_template: EmailTemplatesParams,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct DeleteEmailTemplatesRequest {
    pub account_id: u64,
    pub email_template_id: u64,
}
