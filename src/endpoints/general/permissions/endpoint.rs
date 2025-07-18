use crate::endpoints::general::permissions::{
    GetPermissionResourceResponse, GetPermissionResourcesRequest, UpdateTokenPermissionsRequest,
    UpdateTokenPermissionsResponse,
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for UpdateTokenPermissionsRequest {
    type ResponseType = UpdateTokenPermissionsResponse;

    fn method(&self) -> Method {
        Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "api/accounts/{}/account_accesses/{}/permissions/bulk",
            self.account_id, self.account_access_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self).unwrap();
        Some(RequestBody::Json(body))
    }
}

impl EndpointSpec for GetPermissionResourcesRequest {
    type ResponseType = Vec<GetPermissionResourceResponse>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("api/accounts/{}/permissions/resources", self.account_id)
    }
}
