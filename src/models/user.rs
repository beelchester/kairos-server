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
    pub picture: Option<String>,
    pub u_type: UserPlan,
}

impl User {
    pub fn new(
        user_id: Uuid,
        name: String,
        email: String,
        oauth_provider: Option<OauthProvider>,
        picture: Option<String>,
        u_type: UserPlan,
    ) -> Self {
        Self {
            user_id,
            name,
            email,
            oauth_provider,
            picture,
            u_type,
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

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum UserPlan {
    #[allow(non_camel_case_types)]
    free,
    #[allow(non_camel_case_types)]
    pro,
}

impl Display for UserPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
