use crate::endpoints::contacts::fields::schema::CustomField;
use crate::endpoints::contacts::fields::{
    CreateFieldRequest, DeleteFieldRequest, GetFieldRequest, GetListFieldsRequest,
    UpdateFieldRequest,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for GetListFieldsRequest {
    type ResponseType = Vec<CustomField>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts/fields", self.account_id)
    }
}

impl EndpointSpec for CreateFieldRequest {
    type ResponseType = CustomField;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts/fields", self.account_id)
    }
}

impl EndpointSpec for GetFieldRequest {
    type ResponseType = CustomField;

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

impl EndpointSpec for UpdateFieldRequest {
    type ResponseType = CustomField;

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

impl EndpointSpec for DeleteFieldRequest {
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
