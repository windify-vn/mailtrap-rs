use crate::endpoints::sending::emails::{BatchEmailResponse, SendEmailRequest, SendEmailResponse};
use crate::framework::endpoint::{EndpointSpec, RequestBody, serialize_query};
use http::Method;

impl EndpointSpec for SendEmailRequest {
    type ResponseType = SendEmailResponse;

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
}

impl EndpointSpec for BatchEmailResponse {
    type ResponseType = SendEmailResponse;

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
}
