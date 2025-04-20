use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Session;

/// Add session for the user
pub async fn add_session(pool: web::Data<PgPool>, json: web::Json<Session>) -> impl Responder {
    let session = json.into_inner();
    let result = sqlx::query!(
        "INSERT INTO sessions (session_id, user_id, project_id, started_at, ended_at, duration)
         VALUES ($1, $2, $3, $4, $5, $6)",
        session.session_id,
        session.user_id,
        session.project_id,
        session.started_at,
        session.ended_at,
        session.duration
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Update the session
/// Or also to change the duration of the session (start/end) after the session was ended
/// but the changes after the session was ended will not update the user's focus points and total
/// focus time, it will only be reflected in the personal user stats section.
/// Max duration for any running session is set by the user.. by default 4 hours, can be set upto 6
/// hours
/// Max duration to update any past session is 4 hours
pub async fn update_session(pool: web::Data<PgPool>, json: web::Json<Session>) -> impl Responder {
    let session = json.into_inner();
    let result = sqlx::query!(
        "UPDATE sessions SET ended_at = $1, duration = $2
         WHERE user_id = $3 AND session_id = $4",
        session.ended_at,
        session.duration,
        session.user_id,
        session.session_id
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(rows_affected) if rows_affected.rows_affected() > 0 => HttpResponse::Ok().finish(),
        _ => HttpResponse::NotFound().finish(),
    }
}

/// Check if the active session is already running on another device
/// If found it will be used for syncing purpose
/// TODO: Use this only when app is started after that start a web socket connection from frontend
/// that connection should trigger only if start session or stop session was requested for that
/// user
pub async fn check_active_session(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    let row = sqlx::query!(
        "SELECT session_id, user_id, project_id, started_at, ended_at, duration
         FROM sessions
         WHERE user_id = $1 AND ended_at IS NULL
         LIMIT 1",
        user_id.into_inner()
    )
    .fetch_optional(&**pool)
    .await;

    match row {
        Ok(Some(row)) => {
            let active_session = Session::new(
                row.session_id,
                row.user_id,
                row.project_id,
                row.started_at,
                row.ended_at,
                row.duration,
            );
            HttpResponse::Ok().json(active_session)
        }
        _ => HttpResponse::NotFound().finish(),
    }
}

/// Get all user sessions
pub async fn get_sessions(pool: web::Data<PgPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let rows = sqlx::query!(
        "SELECT session_id, user_id, project_id, started_at, ended_at, duration
         FROM sessions
         WHERE user_id = $1",
        user_id.into_inner()
    )
    .fetch_all(&**pool)
    .await;

    match rows {
        Ok(rows) => {
            let sessions: Vec<Session> = rows
                .into_iter()
                .map(|row| Session {
                    session_id: row.session_id,
                    user_id: row.user_id,
                    project_id: row.project_id,
                    started_at: row.started_at,
                    ended_at: row.ended_at,
                    duration: row.duration,
                })
                .collect();
            HttpResponse::Ok().json(sessions)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
