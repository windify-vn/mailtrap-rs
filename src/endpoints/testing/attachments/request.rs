use crate::endpoints::testing::attachments::AttachmentType;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct GetListAttachmentsRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub inbox_id: u64,
    #[serde(skip)]
    pub message_id: u64,

    #[builder(default, setter(strip_option, into))]
    pub attachment_type: Option<AttachmentType>,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetAttachmentRequest {
    pub account_id: u64,
    pub attachment_id: u64,
    pub inbox_id: u64,
    pub message_id: u64,
}
