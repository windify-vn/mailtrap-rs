use crate::endpoints::testing::projects::response::DeleteProjectResponse;
use crate::endpoints::testing::projects::{
    CreateProjectRequest, DeleteProjectRequest, GetListProjectRequest, GetProjectRequest,
    ProjectDetail, UpdateProjectRequest,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for CreateProjectRequest {
    type ResponseType = ProjectDetail;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/projects", self.account_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for GetListProjectRequest {
    type ResponseType = Vec<ProjectDetail>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/projects", self.account_id)
    }
}

impl EndpointSpec for GetProjectRequest {
    type ResponseType = ProjectDetail;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/projects/{}",
            self.account_id, self.project_id
        )
    }
}

impl EndpointSpec for UpdateProjectRequest {
    type ResponseType = ProjectDetail;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/projects/{}",
            self.account_id, self.project_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for DeleteProjectRequest {
    type ResponseType = DeleteProjectResponse;

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/projects/{}",
            self.account_id, self.project_id
        )
    }
}
