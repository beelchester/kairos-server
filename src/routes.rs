use actix_web::web;

use crate::handlers::{
    add_project, add_session, check_active_session, delete_project, get_projects, get_sessions,
    get_todays_focus_time, health_check, login_user, update_project, update_session,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // auth
        .route("/login/{provider}", web::post().to(login_user))
        // project
        .route("/add_project", web::post().to(add_project))
        .route("/update_project", web::post().to(update_project))
        .route("/delete_project", web::delete().to(delete_project))
        .route("/get_projects/{user_id}", web::get().to(get_projects))
        // session
        .route("/add_session", web::post().to(add_session))
        .route("/update_session", web::post().to(update_session))
        .route(
            "/check_active_session/{user_id}",
            web::get().to(check_active_session),
        )
        .route("/get_sessions/{user_id}", web::get().to(get_sessions))
        // misc
        .route("/health_check", web::get().to(health_check))
        .route(
            "/get_todays_focus_time/{user_id}",
            web::get().to(get_todays_focus_time),
        );
}
