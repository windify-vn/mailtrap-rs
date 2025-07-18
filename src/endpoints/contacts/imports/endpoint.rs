use crate::endpoints::contacts::imports::schema::ContactImportDetail;
use crate::endpoints::contacts::imports::{GetImportContactRequest, ImportContactsRequest};
use crate::framework::endpoint::EndpointSpec;
use http::Method;

impl EndpointSpec for ImportContactsRequest {
    type ResponseType = Vec<ContactImportDetail>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/contacts/imports", self.account_id)
    }
}

impl EndpointSpec for GetImportContactRequest {
    type ResponseType = ContactImportDetail;

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
