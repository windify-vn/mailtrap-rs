use crate::endpoints::contacts::imports::schema::ContactDetail;
use crate::endpoints::contacts::imports::{GetContactRequest, ImportContactsRequest};
use crate::framework::endpoint::EndpointSpec;
use http::Method;

impl EndpointSpec for ImportContactsRequest {
    type ResponseType = Vec<ContactDetail>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts/imports", self.account_id)
    }
}

impl EndpointSpec for GetContactRequest {
    type ResponseType = ContactDetail;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/contacts/imports/{}",
            self.account_id, self.import_id
        )
    }
}
