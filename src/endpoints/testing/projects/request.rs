use crate::endpoints::testing::projects::{CreateProjectParams, UpdateProjectParams};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct CreateProjectRequest {
    #[serde(skip)]
    pub account_id: u64,

    pub project: CreateProjectParams,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetListProjectRequest {
    pub account_id: u64,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct GetProjectRequest {
    pub account_id: u64,
    pub project_id: u64,
}

#[serde_with::skip_serializing_none]
#[derive(TypedBuilder, Serialize, Default, Debug)]
pub struct UpdateProjectRequest {
    #[serde(skip)]
    pub account_id: u64,
    #[serde(skip)]
    pub project_id: u64,

    pub project: UpdateProjectParams,
}

#[derive(TypedBuilder, Default, Debug)]
pub struct DeleteProjectRequest {
    pub account_id: u64,
    pub project_id: u64,
}
