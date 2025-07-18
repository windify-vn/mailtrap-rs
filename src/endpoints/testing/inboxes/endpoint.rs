use crate::endpoints::testing::inboxes::{
    CleanInboxRequest, CreateInboxRequest, DeleteInboxRequest, GetInboxRequest,
    GetListInboxRequest, InboxDetail, MarkAsReadInboxRequest, ResetCredentialInboxRequest,
    ResetEmailInboxRequest, ToggleEmailInboxRequest, UpdateInboxRequest,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for GetInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/inboxes/{}", self.account_id, self.inbox_id)
    }
}

impl EndpointSpec for CreateInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/projects/{}/inboxes",
            self.account_id, self.project_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for DeleteInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/inboxes/{}", self.account_id, self.inbox_id)
    }
}

impl EndpointSpec for UpdateInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/inboxes/{}", self.account_id, self.inbox_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for CleanInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/clean",
            self.account_id, self.inbox_id
        )
    }
}

impl EndpointSpec for MarkAsReadInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/all_read",
            self.account_id, self.inbox_id
        )
    }
}

impl EndpointSpec for ResetCredentialInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/reset_credentials",
            self.account_id, self.inbox_id
        )
    }
}

impl EndpointSpec for ToggleEmailInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/toggle_email_username",
            self.account_id, self.inbox_id
        )
    }
}

impl EndpointSpec for ResetEmailInboxRequest {
    type ResponseType = InboxDetail;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/inboxes/{}/reset_email_username",
            self.account_id, self.inbox_id
        )
    }
}

impl EndpointSpec for GetListInboxRequest {
    type ResponseType = Vec<InboxDetail>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/inboxes", self.account_id)
    }
}
