use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Get today's focused duration
pub async fn get_todays_focus_time(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
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
