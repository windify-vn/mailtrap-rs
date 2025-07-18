use crate::endpoints::testing::attachments::{
    AttachmentDetail, GetAttachmentRequest, GetListAttachmentsRequest,
};
use crate::framework::endpoint::{EndpointSpec, serialize_query};
use http::Method;

impl EndpointSpec for GetListAttachmentsRequest {
    type ResponseType = Vec<AttachmentDetail>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/attachments",
            self.account_id, self.inbox_id, self.message_id
        )
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(self)
    }
}

impl EndpointSpec for GetAttachmentRequest {
    type ResponseType = AttachmentDetail;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/attachments/{}",
            self.account_id, self.inbox_id, self.message_id, self.attachment_id
        )
    }
}
