use crate::endpoints::testing::messages::response::{
    ForwardEmailMessageResponse, GetHtmlAnalysisResponse, GetMessageHeadersResponse,
};
use crate::endpoints::testing::messages::{
    DeleteEmailMessageRequest, ForwardEmailMessageRequest, GetEmailMessageRequest,
    GetEmlMessageRequest, GetHtmlAnalysisRequest, GetHtmlMessageRequest,
    GetListEmailMessageRequest, GetMessageHeadersRequest, GetMessageSourceRequest,
    GetMessageSpamReportRequest, GetRawMessageRequest, GetTextMessageRequest, MessageDetail,
    MessageSpamReport, UpdateEmailMessageRequest,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody, serialize_query};
use http::Method;

impl EndpointSpec for GetEmailMessageRequest {
    type ResponseType = MessageDetail;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for UpdateEmailMessageRequest {
    type ResponseType = MessageDetail;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}",
            self.account_id, self.inbox_id, self.message_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for DeleteEmailMessageRequest {
    type ResponseType = MessageDetail;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for GetListEmailMessageRequest {
    type ResponseType = Vec<MessageDetail>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages",
            self.account_id, self.inbox_id
        )
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(self)
    }
}

impl EndpointSpec for ForwardEmailMessageRequest {
    type ResponseType = ForwardEmailMessageResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/forward",
            self.account_id, self.inbox_id, self.message_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for GetMessageSpamReportRequest {
    type ResponseType = MessageSpamReport;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/spam_report",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for GetHtmlAnalysisRequest {
    type ResponseType = GetHtmlAnalysisResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/analyze",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for GetTextMessageRequest {
    type ResponseType = String;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/body.txt",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for GetRawMessageRequest {
    type ResponseType = String;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/body.raw",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for GetMessageSourceRequest {
    type ResponseType = String;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/body.htmlsource",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for GetHtmlMessageRequest {
    type ResponseType = String;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/body.html",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for GetEmlMessageRequest {
    type ResponseType = String;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/body.eml",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}

impl EndpointSpec for GetMessageHeadersRequest {
    type ResponseType = GetMessageHeadersResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/messages/{}/mail_headers",
            self.account_id, self.inbox_id, self.message_id
        )
    }
}
