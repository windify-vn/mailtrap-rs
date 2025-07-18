use crate::endpoints::bulk::request::{BatchSendBulkEmailRequest, SendBulkEmailRequest};
use crate::endpoints::bulk::response::{BatchSendBulkEmailResponse, SendBulkEmailResponse};
use crate::framework::endpoint::{ApiType, EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for SendBulkEmailRequest {
    type ResponseType = SendBulkEmailResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "api/send".into()
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }

    #[inline]
    fn api_type(&self) -> ApiType {
        ApiType::Bulk
    }
}

impl EndpointSpec for BatchSendBulkEmailRequest {
    type ResponseType = BatchSendBulkEmailResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "api/batch".into()
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }

    #[inline]
    fn api_type(&self) -> ApiType {
        ApiType::Bulk
    }
}
