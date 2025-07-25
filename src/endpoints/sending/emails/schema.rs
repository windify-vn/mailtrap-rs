use crate::endpoints::sending::{Attachment, EmailAddress};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct BatchEmailBase {
    #[builder(default, setter(strip_option, into))]
    pub from: Option<EmailAddress>,

    #[builder(default, setter(strip_option, into))]
    pub reply_to: Option<EmailAddress>,

    #[builder(default, setter(strip_option, into))]
    pub subject: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub html: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub attachments: Option<Vec<Attachment>>,

    #[builder(default, setter(strip_option, into))]
    pub headers: Option<HashMap<String, String>>,
    #[builder(default, setter(strip_option, into))]
    pub category: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub custom_variables: Option<HashMap<String, String>>,
    #[builder(default, setter(strip_option, into))]
    pub template_uuid: Option<uuid::Uuid>,

    #[builder(default, setter(strip_option, into))]
    pub template_variables: Option<serde_json::Value>,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct BatchEmailRequest {
    #[builder(default, setter(strip_option, into))]
    pub from: Option<EmailAddress>,

    #[builder(default, setter(strip_option, into))]
    pub to: Option<Vec<EmailAddress>>,
    #[builder(default, setter(strip_option, into))]
    pub cc: Option<Vec<EmailAddress>>,
    #[builder(default, setter(strip_option, into))]
    pub bcc: Option<Vec<EmailAddress>>,
    #[builder(default, setter(strip_option, into))]
    pub reply_to: Option<EmailAddress>,

    #[builder(default, setter(strip_option, into))]
    pub subject: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub text: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub html: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub attachments: Option<Vec<Attachment>>,

    #[builder(default, setter(strip_option, into))]
    pub headers: Option<HashMap<String, String>>,
    #[builder(default, setter(strip_option, into))]
    pub category: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub custom_variables: Option<HashMap<String, String>>,

    #[builder(default, setter(strip_option, into))]
    pub template_uuid: Option<uuid::Uuid>,
    #[builder(default, setter(strip_option, into))]
    pub template_variables: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct BatchEmailResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub message_ids: Vec<String>,
    #[serde(default)]
    pub errors: Vec<String>,
}
