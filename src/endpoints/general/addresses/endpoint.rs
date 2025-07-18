use crate::endpoints::general::addresses::{
    DeleteAccessRequest, DeleteAccessResponse, GetListAccessesRequest, GetListAccessesResponse,
};
use crate::framework::endpoint::{EndpointSpec, serialize_query};
use http::Method;

impl EndpointSpec for GetListAccessesRequest {
    type ResponseType = Vec<GetListAccessesResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/account_accesses", self.account_id)
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(self)
    }
}

impl EndpointSpec for DeleteAccessRequest {
    type ResponseType = DeleteAccessResponse;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/account_accesses/{}",
            self.account_id, self.account_access_id
        )
    }
}
