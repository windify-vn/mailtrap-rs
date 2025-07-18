use crate::endpoints::sending::domains::{
    CreateDomainRequest, DeleteDomainRequest, GetDomainRequest, ListDomainRequest,
    ListDomainResponse, SendSetupDomainInstructionRequest, SendingDomainDetail,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use RequestBody::Json;
use http::Method;

impl EndpointSpec for CreateDomainRequest {
    type ResponseType = SendingDomainDetail;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/sending_domains", self.account_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(Json(body))
    }
}

impl EndpointSpec for ListDomainRequest {
    type ResponseType = ListDomainResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/sending_domains", self.account_id)
    }
}

impl EndpointSpec for GetDomainRequest {
    type ResponseType = SendingDomainDetail;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/sending_domains/{}",
            self.account_id, self.sending_domain_id
        )
    }
}

impl EndpointSpec for DeleteDomainRequest {
    type ResponseType = ();

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/sending_domains/{}",
            self.account_id, self.sending_domain_id
        )
    }
}

impl EndpointSpec for SendSetupDomainInstructionRequest {
    type ResponseType = ();

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/sending_domains/{}/send_setup_instructions",
            self.account_id, self.sending_domain_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(Json(body))
    }
}
