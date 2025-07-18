use crate::endpoints::contacts::lists::schema::ContactList;
use crate::endpoints::contacts::lists::{
    CreateContactListsRequest, DeleteContactListsRequest, GetContactListsRequest,
    GetListContactListsRequest, UpdateContactListsRequest,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for GetListContactListsRequest {
    type ResponseType = Vec<ContactList>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts/lists", self.account_id)
    }
}

impl EndpointSpec for CreateContactListsRequest {
    type ResponseType = ContactList;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts/lists", self.account_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for GetContactListsRequest {
    type ResponseType = ContactList;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/lists/{}",
            self.account_id, self.list_id
        )
    }
}

impl EndpointSpec for UpdateContactListsRequest {
    type ResponseType = ContactList;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/lists/{}",
            self.account_id, self.list_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for DeleteContactListsRequest {
    type ResponseType = ();

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/lists/{}",
            self.account_id, self.list_id
        )
    }
}
