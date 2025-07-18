use crate::endpoints::sending::emails::{BatchSendEmailResponse, SendEmailResponse};
use crate::endpoints::testing::emails::{BatchSendEmailRequest, SendEmailRequest};
use crate::framework::endpoint::{ApiType, EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for SendEmailRequest {
    type ResponseType = SendEmailResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/send/{}", self.inbox_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }

    #[inline]
    fn api_type(&self) -> ApiType {
        ApiType::Sandbox
    }
}

impl EndpointSpec for BatchSendEmailRequest {
    type ResponseType = BatchSendEmailResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/batch/{}", self.inbox_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }

    #[inline]
    fn api_type(&self) -> ApiType {
        ApiType::Sandbox
    }
}
