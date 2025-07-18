pub mod auth;
pub mod client;
pub mod endpoint;
pub mod response;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error via the `reqwest` crate
    #[error("Reqwest returned an error when connecting to the Mailgun API: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug)]
pub enum Environment {
    Production,
    Mock,
    Custom(String),
}

impl From<&Environment> for url::Url {
    fn from(env: &Environment) -> Self {
        match env {
            Environment::Production => url::Url::parse("https://mailtrap.io/").unwrap(),
            Environment::Mock => {
                url::Url::parse("https://stoplight.io/mocks/railsware/mailtrap-api-docs/100824079/")
                    .unwrap()
            }
            Environment::Custom(url) => url::Url::parse(url.as_str()).unwrap(),
        }
    }
}
