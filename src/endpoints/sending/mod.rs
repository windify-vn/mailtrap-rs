pub mod domains;
pub mod emails;

use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fmt;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Default, Debug, PartialEq, Eq, Clone)]
pub struct EmailAddress {
    #[builder(setter(into))]
    pub email: String,

    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.name {
            Some(ref name) => write!(f, "{} <{}>", name, self.email),
            None => write!(f, "{}", self.email),
        }
    }
}

impl From<&str> for EmailAddress {
    fn from(email: &str) -> Self {
        EmailAddress::builder().email(email).build()
    }
}

impl From<(&str, &str)> for EmailAddress {
    fn from((name, email): (&str, &str)) -> Self {
        EmailAddress::builder().email(email).name(name).build()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Disposition {
    Inline,
    #[default]
    Attachment,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Attachment {
    pub content: String,
    #[serde(rename = "type")]
    pub mime_type: Option<String>,
    pub filename: String,
    pub disposition: Option<Disposition>,
    pub content_id: Option<String>,
}

impl Attachment {
    #[allow(dead_code)]
    pub fn from_file<P: AsRef<std::path::Path>>(file_path: P) -> Result<Self, std::io::Error> {
        let path = file_path.as_ref();

        let file_bytes = std::fs::read(path)?;

        let encoded_content = base64::engine::general_purpose::STANDARD.encode(&file_bytes);

        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("untitled")
            .to_string();

        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        Ok(Attachment {
            content: encoded_content,
            mime_type: Some(mime_type),
            filename,
            ..Default::default()
        })
    }
}
