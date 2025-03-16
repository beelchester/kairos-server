use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use sqlx::{types::chrono, PgPool};
use uuid::Uuid;

mod db;

const UNIQUE_VIOLATION: &str = "23505";

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct User {
    #[serde(rename = "userId")]
    user_id: String,
    name: String,
    email: String,
}

impl User {
    fn new(user_id: String, name: String, email: String) -> Self {
        Self {
            user_id,
            name,
            email,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]

struct Project {
    #[serde(rename = "projectId")]
    project_id: Uuid,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "projectName")]
    project_name: String,
    colour: String,
    deadline: Option<DateTime<Utc>>,
    priority: Option<i32>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Session {
    #[serde(rename = "sessionId")]
    session_id: Uuid,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "projectId")]
    project_id: Uuid,
    #[serde(rename = "startedAt")]
    started_at: DateTime<Utc>,
    #[serde(rename = "endedAt")]
    ended_at: Option<DateTime<Utc>>,
    duration: i32,
}

impl Project {
    fn new(
        user_id: String,
        project_id: Uuid,
        project_name: String,
        colour: String,
        deadline: Option<DateTime<Utc>>,
        priority: Option<i32>,
    ) -> Self {
        Self {
            user_id,
            project_id,
            project_name,
            colour,
            deadline,
            priority,
        }
    }
}

impl Session {
    fn new(
        session_id: Uuid,
        user_id: String,
        project_id: Uuid,
        started_at: DateTime<Utc>,
        ended_at: Option<DateTime<Utc>>,
        duration: i32,
    ) -> Self {
        Self {
            session_id,
            user_id,
            project_id,
            started_at,
            ended_at,
            duration,
        }
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Signup user
async fn add_user(pool: web::Data<PgPool>, json: web::Json<User>) -> impl Responder {
    let user = json.into_inner();
    let result = sqlx::query!(
        "INSERT INTO users (user_id, name, email)
         VALUES ($1, $2, $3)",
        user.user_id,
        user.name,
        user.email,
    )
    .execute(&**pool)
    .await;

    // Default project called "Unset"
    let default_project = Project::new(
        user.user_id.clone(),
        Uuid::new_v4(),
        "Unset".to_string(),
        "grey".to_string(),
        None,
        None,
    );

    let add_project = sqlx::query!(
        "INSERT INTO projects (project_id, user_id, project_name, colour, deadline)
         VALUES ($1, $2, $3, $4, $5)",
        default_project.project_id,
        user.user_id.clone(),
        default_project.project_name,
        default_project.colour,
        default_project.deadline,
    );

    match result {
        Ok(_) => match add_project.execute(&**pool).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        // handle user already exist
        Err(e) => match e {
            sqlx::Error::Database(err) => {
                if err.code() == Some(UNIQUE_VIOLATION.into()) {
                    HttpResponse::Conflict().finish()
                } else {
                    HttpResponse::InternalServerError().finish()
                }
            }
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

/// Add project for the user
async fn add_project(pool: web::Data<PgPool>, json: web::Json<Project>) -> impl Responder {
    let project = json.into_inner();
    let result = sqlx::query!(
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
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => match e {
            sqlx::Error::Database(err) => {
                if err.code() == Some(UNIQUE_VIOLATION.into()) {
                    HttpResponse::Conflict().finish()
                } else {
                    HttpResponse::InternalServerError().finish()
                }
            }
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

/// Update project
async fn update_project(pool: web::Data<PgPool>, json: web::Json<Project>) -> impl Responder {
    let project = json.into_inner();
    let result = sqlx::query!(
        "UPDATE projects SET project_name = $1, colour = $2, deadline = $3, priority = $4
         WHERE user_id = $5 AND project_id = $6",
        project.project_name,
        project.colour,
        project.deadline,
        project.priority,
        project.user_id,
        project.project_id
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(rows_affected) if rows_affected.rows_affected() > 0 => HttpResponse::Ok().finish(),
        _ => HttpResponse::NotFound().finish(),
    }
}

/// Delete the project
async fn delete_project(pool: web::Data<PgPool>, json: web::Json<Project>) -> impl Responder {
    let project = json.into_inner();
    let result = sqlx::query!(
        "DELETE FROM projects WHERE project_id = $1 AND user_id = $2",
        project.project_id,
        project.user_id
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(rows_affected) if rows_affected.rows_affected() > 0 => HttpResponse::Ok().finish(),
        _ => HttpResponse::NotFound().finish(),
    }
}

/// Get all user projects
async fn get_projects(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
    let rows = sqlx::query!(
        "SELECT project_id, user_id, project_name, colour, deadline, priority
         FROM projects
         WHERE user_id = $1",
        user_id.into_inner()
    )
    .fetch_all(&**pool)
    .await;

    match rows {
        Ok(rows) => {
            let projects: Vec<Project> = rows
                .into_iter()
                .map(|row| Project {
                    project_id: row.project_id,
                    user_id: row.user_id,
                    project_name: row.project_name,
                    colour: row.colour,
                    deadline: row.deadline,
                    priority: row.priority,
                })
                .collect();
            HttpResponse::Ok().json(projects)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Add session for the user
async fn add_session(pool: web::Data<PgPool>, json: web::Json<Session>) -> impl Responder {
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
async fn update_session(pool: web::Data<PgPool>, json: web::Json<Session>) -> impl Responder {
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
async fn check_active_session(
    pool: web::Data<PgPool>,
    user_id: web::Path<String>,
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
                row.user_id.to_string(),
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
async fn get_sessions(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
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

// TODO:
/// Get all users ranked by their focus points
//
// async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
// let rows = sqlx::query!(
//     "SELECT user_id, name, email, total_time, created_at, updated_at
//      FROM users"
// )
// .fetch_all(&**pool)
// .await;
//
// match rows {
//     Ok(users) => HttpResponse::Ok().json(users),
//     Err(_) => HttpResponse::InternalServerError().finish(),
// }
// }

async fn get_user(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
    let row = sqlx::query!(
        "SELECT user_id, name, email
         FROM users
         WHERE user_id = $1",
        user_id.into_inner()
    )
    .fetch_optional(&**pool)
    .await;

    match row {
        Ok(Some(user)) => {
            let user = User::new(user.user_id, user.name, user.email);
            HttpResponse::Ok().json(user)
        }
        _ => HttpResponse::NotFound().finish(),
    }
}

/// Delete the user and the sessions linked to the user
async fn delete_user(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
    let user_table_result = sqlx::query!("DELETE FROM users WHERE user_id = $1", user_id.clone())
        .execute(&**pool)
        .await;

    match user_table_result {
        Ok(rows_affected) if rows_affected.rows_affected() > 0 => HttpResponse::Ok().finish(),
        _ => HttpResponse::NotFound().finish(),
    }
}

/// Get today's focused duration
async fn get_todays_focus_time(
    pool: web::Data<PgPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    let today = chrono::Utc::now().date_naive();
    let rows = sqlx::query!(
        "SELECT session_id, user_id, started_at, ended_at, duration
         FROM sessions
         WHERE user_id = $1 AND DATE(started_at) = $2",
        user_id.into_inner(),
        today
    )
    .fetch_all(&**pool)
    .await;

    match rows {
        Ok(sessions) => {
            let mut total_duration = 0;
            sessions
                .iter()
                .for_each(|ses| total_duration += ses.duration);
            HttpResponse::Ok().json(total_duration)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    dotenv().ok();

    let pool = db::create_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:6080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::ACCEPT,
                    ])
                    .supports_credentials(),
            )
            .app_data(web::Data::new(pool.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/add_user", web::post().to(add_user))
            .route("/add_project", web::post().to(add_project))
            .route("/update_project", web::post().to(update_project))
            .route("/delete_project", web::delete().to(delete_project))
            .route("/get_projects/{user_id}", web::get().to(get_projects))
            .route("/add_session", web::post().to(add_session))
            .route("/update_session", web::post().to(update_session))
            .route(
                "/check_active_session/{user_id}",
                web::get().to(check_active_session),
            )
            .route("/get_sessions/{user_id}", web::get().to(get_sessions))
            // .route("/get_users", web::get().to(get_users))
            .route("/get_user/{user_id}", web::get().to(get_user))
            .route("/delete_user/{user_id}", web::delete().to(delete_user))
            .route(
                "/get_todays_focus_time/{user_id}",
                web::get().to(get_todays_focus_time),
            )
    })
    .listen(listener)?
    .run()
    .await
}
