#[derive(Clone, Debug)]
pub enum Credentials {
    UserAuthKey { key: String },
    UserAuthToken { token: String },
}

impl Credentials {
    pub fn headers(&self) -> Vec<(&'static str, String)> {
        match self {
            Self::UserAuthKey { key } => {
                vec![("Api-Token", key.clone())]
            }
            Self::UserAuthToken { token } => {
                vec![("Authorization", format!("Bearer {}", token.clone()))]
            }
        }
    }
}

pub trait AuthClient {
    fn auth(self, credentials: &Credentials) -> Self;
}
