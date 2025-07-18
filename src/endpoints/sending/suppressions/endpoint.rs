use crate::endpoints::sending::suppressions::{
    DeleteSuppressionsRequest, ListSuppressionsRequest, SuppressionDetail,
};
use crate::framework::endpoint::{EndpointSpec, serialize_query};
use http::Method;

impl EndpointSpec for ListSuppressionsRequest {
    type ResponseType = Vec<SuppressionDetail>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/suppressions", self.account_id)
    }

    #[inline]
    fn query(&self) -> Option<String> {
        serialize_query(self)
    }
}

impl EndpointSpec for DeleteSuppressionsRequest {
    type ResponseType = SuppressionDetail;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/suppressions/{}",
            self.account_id, self.suppression_id
        )
    }
}
