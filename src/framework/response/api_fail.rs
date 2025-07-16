use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct ApiErrors(serde_json::Value);

impl ApiErrors {
    pub fn into_inner(self) -> serde_json::Value {
        self.0
    }
}

impl AsRef<serde_json::Value> for ApiErrors {
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}

#[derive(Debug)]
pub enum ApiFailure {
    Error(reqwest::StatusCode, ApiErrors),
    Invalid(reqwest::Error),
}

impl Error for ApiFailure {}

impl PartialEq for ApiFailure {
    fn eq(&self, other: &ApiFailure) -> bool {
        match (self, other) {
            (ApiFailure::Invalid(e1), ApiFailure::Invalid(e2)) => e1.to_string() == e2.to_string(),
            (ApiFailure::Error(status1, e1), ApiFailure::Error(status2, e2)) => {
                status1 == status2 && e1 == e2
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
                write!(f, "HTTP {status} - {:?}", api_errors.as_ref())
            }
            ApiFailure::Invalid(err) => write!(f, "{err}"),
        }
    }
}

impl From<reqwest::Error> for ApiFailure {
    fn from(error: reqwest::Error) -> Self {
        ApiFailure::Invalid(error)
    }
}
