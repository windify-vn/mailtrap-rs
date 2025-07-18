use crate::endpoints::sending::emails::{BatchEmailBase, BatchEmailRequest};
use crate::endpoints::sending::{Attachment, EmailAddress};
use serde::Serialize;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct SendBulkEmailRequest {
    #[builder(setter(into))]
    pub from: EmailAddress,

    #[builder(setter(into))]
    pub subject: String,

    #[builder(default, setter(strip_option, into))]
    pub to: Option<Vec<EmailAddress>>,
    #[builder(default, setter(strip_option, into))]
    pub cc: Option<Vec<EmailAddress>>,
    #[builder(default, setter(strip_option, into))]
    pub bbc: Option<Vec<EmailAddress>>,
    #[builder(default, setter(strip_option, into))]
    pub reply_to: Option<EmailAddress>,

    #[builder(default, setter(strip_option, into))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub html: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub template_uuid: Option<uuid::Uuid>,
    #[builder(default, setter(strip_option, into))]
    pub template_variables: Option<serde_json::Value>,

    #[builder(default, setter(strip_option, into))]
    pub category: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub attachments: Option<Vec<Attachment>>,

    #[builder(default, setter(strip_option, into))]
    pub headers: Option<HashMap<String, String>>,
    #[builder(default, setter(strip_option, into))]
    pub custom_variables: Option<HashMap<String, String>>,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct BatchSendBulkEmailRequest {
    #[builder(default, setter(strip_option, into))]
    pub base: Option<BatchEmailBase>,
    pub requests: Vec<BatchEmailRequest>,
}
