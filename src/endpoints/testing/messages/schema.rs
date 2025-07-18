use crate::framework::response::JsonResult;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageDetail {
    pub id: i64,
    pub inbox_id: i64,
    pub subject: String,
    pub sent_at: String,
    pub from_email: String,
    pub from_name: String,
    pub to_email: String,
    pub to_name: String,
    pub email_size: i64,
    pub is_read: bool,
    pub created_at: String,
    pub updated_at: String,
    pub html_body_size: i64,
    pub text_body_size: i64,
    pub human_size: String,
    pub html_path: String,
    pub txt_path: String,
    pub raw_path: String,
    pub download_path: String,
    pub html_source_path: String,
    #[serde(flatten)]
    pub blacklists_report_info: BlacklistReportInfo,
    pub smtp_information: SmtpInformation,
}

impl JsonResult for MessageDetail {}
impl JsonResult for Vec<MessageDetail> {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlacklistsReportInfo {
    pub result: ResultStatus,
    pub domain: String,
    pub ip: String,
    pub report: Vec<BlacklistReportItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlacklistReportItem {
    pub name: String,
    pub url: String,
    pub in_black_list: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmtpInformation {
    pub ok: bool,
    pub data: SmtpData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmtpData {
    pub mail_from_addr: String,
    pub client_ip: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResultStatus {
    Success,
    Pending,
    Error,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum BlacklistReportInfo {
    Boolean(bool),
    Info(BlacklistsReportInfo),
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct UpdateEmailMessageParams {
    #[serde(serialize_with = "bool_to_string")]
    pub is_read: bool,
}

pub fn bool_to_string<S>(x: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(if *x { "true" } else { "false" })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSpamReport {
    #[serde(rename = "ResponseCode")]
    pub response_code: i32,

    #[serde(rename = "ResponseMessage")]
    pub response_message: String,

    #[serde(rename = "ResponseVersion")]
    pub response_version: String,

    #[serde(rename = "Score")]
    pub score: f64,

    #[serde(rename = "Spam")]
    pub spam: bool,

    #[serde(rename = "Threshold")]
    pub threshold: f64,

    #[serde(rename = "Details")]
    pub details: Vec<SpamReportDetail>,
}

impl JsonResult for MessageSpamReport {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpamReportDetail {
    #[serde(rename = "Pts")]
    pub pts: f64,

    #[serde(rename = "RuleName")]
    pub rule_name: String,

    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlReport {
    pub status: String,
    pub errors: Vec<HtmlError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlError {
    pub error_line: u32,
    pub rule_name: String,
    pub email_clients: EmailClients,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmailClients {
    #[serde(default)]
    pub desktop: Vec<String>,

    #[serde(default)]
    pub mobile: Vec<String>,

    #[serde(default)]
    pub web: Vec<String>,
}
