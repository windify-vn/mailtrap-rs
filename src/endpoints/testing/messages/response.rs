use crate::endpoints::testing::messages::HtmlReport;
use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardEmailMessageResponse {
    pub message: String,
}

impl JsonResult for ForwardEmailMessageResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetHtmlAnalysisResponse {
    pub report: HtmlReport,
}

impl JsonResult for GetHtmlAnalysisResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMessageHeadersResponse {
    pub headers: HashMap<String, String>,
}
impl JsonResult for GetMessageHeadersResponse {}
