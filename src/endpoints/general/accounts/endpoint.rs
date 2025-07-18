use crate::endpoints::general::accounts::{GetAccountsResponse, GetListAccountsRequest};
use crate::framework::endpoint::EndpointSpec;
use http::Method;

impl EndpointSpec for GetListAccountsRequest {
    type ResponseType = Vec<GetAccountsResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        "api/accounts".into()
    }
}
