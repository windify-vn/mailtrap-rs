use crate::endpoints::contacts::fields::ContactField;
use crate::endpoints::contacts::fields::{
    CreateContactFieldRequest, DeleteContactFieldRequest, GetContactFieldRequest,
    GetListContactFieldsRequest, UpdateContactFieldRequest,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for GetListContactFieldsRequest {
    type ResponseType = Vec<ContactField>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts/fields", self.account_id)
    }
}

impl EndpointSpec for CreateContactFieldRequest {
    type ResponseType = ContactField;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts/fields", self.account_id)
    }
}

impl EndpointSpec for GetContactFieldRequest {
    type ResponseType = ContactField;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/fields/{}",
            self.account_id, self.field_id
        )
    }
}

impl EndpointSpec for UpdateContactFieldRequest {
    type ResponseType = ContactField;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/fields/{}",
            self.account_id, self.field_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for DeleteContactFieldRequest {
    type ResponseType = ();

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/fields/{}",
            self.account_id, self.field_id
        )
    }
}
