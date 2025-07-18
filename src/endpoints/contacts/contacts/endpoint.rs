use crate::endpoints::contacts::contacts::{
    CreateContactRequest, DeleteContactRequest, GetContactRequest, UpdateContactRequest,
};
use crate::endpoints::contacts::contacts::{
    CreateContactResponse, GetContactResponse, UpdateContactResponse,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for CreateContactRequest {
    type ResponseType = CreateContactResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts", self.account_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for GetContactRequest {
    type ResponseType = GetContactResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/{}",
            self.account_id, self.contact_id
        )
    }
}

impl EndpointSpec for UpdateContactRequest {
    type ResponseType = UpdateContactResponse;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/{}",
            self.account_id, self.contact_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for DeleteContactRequest {
    type ResponseType = ();

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/{}",
            self.account_id, self.contact_id
        )
    }
}
