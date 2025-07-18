use crate::endpoints::template::{
    CreateEmailTemplatesRequest, DeleteEmailTemplatesRequest, EmailTemplate,
    GetEmailTemplatesRequest, GetListEmailTemplatesRequest, UpdateEmailTemplatesRequest,
};
use crate::framework::endpoint::RequestBody::Json;
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for GetListEmailTemplatesRequest {
    type ResponseType = Vec<EmailTemplate>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/email_templates", self.account_id)
    }
}

impl EndpointSpec for CreateEmailTemplatesRequest {
    type ResponseType = EmailTemplate;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/email_templates", self.account_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(Json(body))
    }
}

impl EndpointSpec for GetEmailTemplatesRequest {
    type ResponseType = EmailTemplate;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/email_templates/{}",
            self.account_id, self.email_template_id
        )
    }
}

impl EndpointSpec for UpdateEmailTemplatesRequest {
    type ResponseType = EmailTemplate;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/email_templates/{}",
            self.account_id, self.email_template_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(Json(body))
    }
}

impl EndpointSpec for DeleteEmailTemplatesRequest {
    type ResponseType = ();

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/email_templates/{}",
            self.account_id, self.email_template_id
        )
    }
}
