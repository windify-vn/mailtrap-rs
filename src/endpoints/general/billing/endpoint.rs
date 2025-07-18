use crate::endpoints::general::billing::{GetAccountBillingRequest, GetAccountBillingResponse};
use crate::framework::endpoint::EndpointSpec;
use http::Method;

impl EndpointSpec for GetAccountBillingRequest {
    type ResponseType = GetAccountBillingResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/billing/usage", self.account_id)
    }
}
