use actix_web::web;
use chrono::{DateTime, Utc};
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

use crate::models::{OauthProvider, Project, User, UserPlan};

pub async fn create_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error connecting to the database")
}

// pub async fn get_client(pool: &PgPool) -> PgPool {
//     pool.clone()
// }

// user
pub async fn create_user(
    pool: web::Data<PgPool>,
    user: User,
) -> Result<PgQueryResult, sqlx::Error> {
    let oauth_provider = user.oauth_provider.map(|provider| provider.to_string());

    sqlx::query!(
        "INSERT INTO users (user_id, name, email, oauth_provider, picture, user_type)
         VALUES ($1, $2, $3, $4, $5, $6)",
        user.user_id,
        user.name,
        user.email,
        oauth_provider,
        user.picture,
        user.u_type.to_string()
    )
    .execute(&**pool)
    .await
}

pub async fn get_user(pool: web::Data<PgPool>, user_email: String) -> Result<User, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT user_id, name, email, oauth_provider, picture
         FROM users
         WHERE email = $1",
        user_email
    )
    .fetch_optional(&**pool)
    .await;
    if let Ok(Some(user)) = row {
        Ok(User::new(
            user.user_id,
            user.name,
            user.email,
            user.oauth_provider.map(|_p| OauthProvider::google),
            user.picture,
            UserPlan::free,
        ))
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

// tokens

pub async fn store_refresh_token(
    pool: web::Data<PgPool>,
    u_id: Uuid,
    token_id: Uuid,
    refresh_token: String,
    expires_at: DateTime<Utc>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO refresh_tokens (token_id, user_id, refresh_token, expires_at )
         VALUES ($1, $2, $3, $4)",
        token_id,
        u_id,
        refresh_token,
        expires_at
    )
    .execute(&**pool)
    .await
}

pub async fn add_project(
    pool: web::Data<PgPool>,
    project: Project,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO projects (project_id, user_id, project_name, colour, deadline, priority)
         VALUES ($1, $2, $3, $4, $5, $6)",
        project.project_id,
        project.user_id,
        project.project_name,
        project.colour,
        project.deadline,
        project.priority,
    )
    .execute(&**pool)
    .await
}
