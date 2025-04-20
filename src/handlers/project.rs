use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::UNIQUE_VIOLATION, models::Project};

/// Add project for the user
pub async fn add_project(pool: web::Data<PgPool>, json: web::Json<Project>) -> impl Responder {
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
pub async fn update_project(pool: web::Data<PgPool>, json: web::Json<Project>) -> impl Responder {
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
pub async fn delete_project(pool: web::Data<PgPool>, json: web::Json<Project>) -> impl Responder {
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
pub async fn get_projects(pool: web::Data<PgPool>, user_id: web::Path<Uuid>) -> impl Responder {
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
