use crate::endpoints::testing::messages::HtmlReport;
use crate::framework::response::ApiResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardEmailMessageResponse {
    pub message: String,
}

impl ApiResult for ForwardEmailMessageResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetHtmlAnalysisResponse {
    pub report: HtmlReport,
}

impl ApiResult for GetHtmlAnalysisResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMessageHeadersResponse {
    pub headers: HashMap<String, String>,
}
impl ApiResult for GetMessageHeadersResponse {}
