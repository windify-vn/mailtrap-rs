use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct ApiErrors {
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default, flatten)]
    pub errors: Option<ErrorValue>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ErrorValue {
    String(String),
    ArrayString(Vec<String>),
    Object(serde_json::Value),
}

#[derive(Debug)]
pub enum ApiFailure {
    Error(reqwest::StatusCode, ApiErrors),
    Invalid(reqwest::Error),
    Decoding(serde_json::Error),
}

impl Error for ApiFailure {}

impl PartialEq for ApiFailure {
    fn eq(&self, other: &ApiFailure) -> bool {
        match (self, other) {
            (ApiFailure::Invalid(e1), ApiFailure::Invalid(e2)) => e1.to_string() == e2.to_string(),
            (ApiFailure::Error(status1, e1), ApiFailure::Error(status2, e2)) => {
                status1 == status2 && e1 == e2
            }
            (ApiFailure::Decoding(e1), ApiFailure::Decoding(e2)) => {
                e1.to_string() == e2.to_string()
            }
            _ => false,
        }
    }
}
impl Eq for ApiFailure {}

impl fmt::Display for ApiFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiFailure::Error(status, api_errors) => {
                write!(f, "HTTP {status} - {api_errors:?}",)
            }
            ApiFailure::Invalid(err) => write!(f, "{err}"),
            ApiFailure::Decoding(err) => write!(f, "Decoding Error - {err}"),
        }
    }
}

impl From<reqwest::Error> for ApiFailure {
    fn from(error: reqwest::Error) -> Self {
        ApiFailure::Invalid(error)
    }
}
