use crate::endpoints::testing::messages::UpdateEmailMessageParams;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Debug)]
pub struct GetEmailMessageRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct UpdateEmailMessageRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub inbox_id: u64,
    #[serde(skip)]
    pub message_id: u64,

    pub message: UpdateEmailMessageParams,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct DeleteEmailMessageRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct GetListEmailMessageRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub inbox_id: u64,

    #[builder(default, setter(strip_option, into))]
    pub last_id: Option<u64>,
    #[builder(default, setter(strip_option, into))]
    pub page: Option<u64>,
    #[builder(default, setter(strip_option, into))]
    pub search: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct ForwardEmailMessageRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub inbox_id: u64,
    #[serde(skip)]
    pub message_id: u64,

    #[builder(setter(into))]
    pub email: String,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetMessageSpamReportRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetHtmlAnalysisRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetTextMessageRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetRawMessageRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetMessageSourceRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetHtmlMessageRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetEmlMessageRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetMessageHeadersRequest {
    pub account_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}
