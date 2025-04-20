use std::fmt::Display;

use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(rename = "oauthProvider")]
    pub oauth_provider: Option<OauthProvider>,
    #[serde(rename = "oauthSub")]
    pub oauth_sub: Option<String>,
    pub picture: Option<String>,
}

impl User {
    pub fn new(
        user_id: Uuid,
        name: String,
        email: String,
        oauth_provider: Option<OauthProvider>,
        oauth_sub: Option<String>,
        picture: Option<String>,
    ) -> Self {
        Self {
            user_id,
            name,
            email,
            oauth_provider,
            oauth_sub,
            picture,
        }
    }
}

#[derive(Debug)]
pub struct OauthUser {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub picture: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum OauthProvider {
    #[allow(non_camel_case_types)]
    google,
    #[allow(non_camel_case_types)]
    github,
}

impl Display for OauthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(serde::Serialize, Debug)]
pub struct LoginResponse {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
}
