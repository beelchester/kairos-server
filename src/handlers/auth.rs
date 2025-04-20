use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpMessage,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    config::{jwt_access_secret, jwt_refresh_secret},
    models::{Claims, OauthUser, TokenResponse},
};

pub fn create_jwt_tokens(user_id: &Uuid) -> Result<TokenResponse, jsonwebtoken::errors::Error> {
    // (Refresh token rotation)
    // Create new Refresh and Access token everytime there's a need to create new access token
    let a_expiry = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let r_expiry = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp");

    let a_claims = Claims {
        sub: user_id.to_string(),
        exp: a_expiry,
    };

    let r_claims = Claims {
        sub: user_id.to_string(),
        exp: r_expiry.timestamp() as usize,
    };

    let header = jsonwebtoken::Header::default();

    let access_token = jsonwebtoken::encode(
        &header,
        &a_claims,
        &jsonwebtoken::EncodingKey::from_secret(&jwt_access_secret()),
    )?;

    let refresh_token = jsonwebtoken::encode(
        &header,
        &r_claims,
        &jsonwebtoken::EncodingKey::from_secret(&jwt_refresh_secret()),
    )?;
    let res = TokenResponse {
        access_token,
        refresh_token,
        expiry: r_expiry,
    };
    Ok(res)
}

pub async fn jwt_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let ignore_jwt = ["/login"];

    // Skip JWT check for ignored paths
    if ignore_jwt.iter().any(|pat| req.path().starts_with(pat)) {
        return next.call(req).await;
    }

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid token"))?;

    // Decode JWT token
    let decoded = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(&jwt_access_secret()),
        &Validation::default(),
    )
    .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // Store user claims and continue
    req.extensions_mut().insert(decoded.claims.sub);
    next.call(req).await
}

pub async fn fetch_google_user_info(
    id_token: &str,
) -> Result<OauthUser, Box<dyn std::error::Error>> {
    let user_info_url = format!(
        "https://oauth2.googleapis.com/tokeninfo?id_token={}",
        id_token, // will verify the token, if valid it will provide the user details
    );

    let client = reqwest::Client::new();
    let response = client.get(user_info_url).send().await?.error_for_status()?;

    let user_info: Value = response.json().await?;
    let sub = user_info["sub"] // unique identifier of the google account
        .as_str()
        .ok_or("Missing 'sub' field")?
        .to_string();
    let picture = user_info["picture"]
        .as_str()
        .ok_or("Missing 'picture' field")?
        .to_string();
    let email = user_info["email"]
        .as_str()
        .ok_or("Missing 'email' field")?
        .to_string();
    let name = user_info["name"]
        .as_str()
        .ok_or("Missing 'name' field")?
        .to_string();

    let _aud = user_info["aud"]
        .as_str()
        .ok_or("Missing 'aud' field")?
        .to_string();
    //TODO: aud needs to be verified with the app id
    // https://developers.google.com/identity/sign-in/android/backend-auth

    Ok(OauthUser {
        sub,
        name,
        email,
        picture,
    })
}
