use crate::endpoints::testing::inboxes::InboxDetail;
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDetail {
    pub id: i64,
    pub name: String,
    pub share_links: ShareLinks,
    pub inboxes: Vec<InboxDetail>,
    pub permissions: ProjectPermissions,
}

impl ApiResult for ProjectDetail {}
impl ApiResult for Vec<ProjectDetail> {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareLinks {
    pub admin: String,
    pub viewer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPermissions {
    pub can_read: bool,
    pub can_update: bool,
    pub can_destroy: bool,
    pub can_leave: bool,
}

#[derive(TypedBuilder, Debug, Clone, Default, Serialize)]
pub struct CreateProjectParams {
    #[builder(setter(into))]
    pub name: String,
}

#[derive(TypedBuilder, Debug, Clone, Default, Serialize)]
pub struct UpdateProjectParams {
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
}
