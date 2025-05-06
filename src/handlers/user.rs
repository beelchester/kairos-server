use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::UNIQUE_VIOLATION,
    db,
    handlers::create_jwt_tokens,
    models::{LoginResponse, OauthProvider, OauthUser, Project, User, UserPlan},
};

/// Login user, create user if needed
pub async fn login_user(
    pool: web::Data<PgPool>,
    provider: web::Path<OauthProvider>,
    json: web::Json<OauthUser>,
) -> impl Responder {
    let g_user = json.into_inner();

    // Support more later
    match provider.into_inner() {
        OauthProvider::google => {
            // Creating user if not exist
            let user = User::new(
                Uuid::new_v4(),
                g_user.name.clone(),
                g_user.email.clone(),
                Some(OauthProvider::google),
                Some(g_user.picture),
                UserPlan::free,
            );

            let res = db::create_user(pool.clone(), user.clone()).await;

            // Default project called "Unset"
            let default_project = Project::new(
                user.user_id,
                Uuid::new_v4(),
                "Unset".to_string(),
                "grey".to_string(),
                None,
                None,
            );

            // if user not exist, create user and add default project
            if res.is_ok() {
                if (db::add_project(pool.clone(), default_project).await).is_ok() {
                    if let Ok(token) = create_jwt_tokens(&user.user_id.clone()) {
                        let login_response = LoginResponse {
                            user: user.clone(),
                            access_token: token.access_token,
                            refresh_token: token.refresh_token.clone(),
                        };
                        let token_id = Uuid::new_v4();
                        let r_token_hash = bcrypt::hash(token.refresh_token, 10);
                        if let Ok(hash) = r_token_hash {
                            let r = db::store_refresh_token(
                                pool,
                                user.user_id,
                                token_id,
                                hash,
                                token.expiry,
                            )
                            .await;
                            if r.is_ok() {
                                return HttpResponse::Ok().json(login_response);
                            } else {
                                return HttpResponse::InternalServerError().finish();
                            }
                        } else {
                            return HttpResponse::InternalServerError().finish();
                        }
                    } else {
                        return HttpResponse::InternalServerError().finish();
                    }
                } else {
                    return HttpResponse::InternalServerError().finish();
                }
            } else if let Err(e) = res {
                match e {
                    sqlx::Error::Database(err) => {
                        if err.code() == Some(UNIQUE_VIOLATION.into()) {
                            // User already exist
                            let user = db::get_user(pool, user.email).await;
                            if let Ok(u) = user {
                                if let Ok(token) = create_jwt_tokens(&u.user_id) {
                                    let login_response = LoginResponse {
                                        user: u,
                                        access_token: token.access_token,
                                        refresh_token: token.refresh_token,
                                    };
                                    return HttpResponse::Ok().json(login_response);
                                } else {
                                    return HttpResponse::InternalServerError().finish();
                                }
                            } else {
                                return HttpResponse::NotFound().finish();
                            }
                        } else {
                            return HttpResponse::InternalServerError().finish();
                        }
                    }
                    _ => return HttpResponse::InternalServerError().finish(),
                }
            }
            return HttpResponse::InternalServerError().finish();
        }
        _ => HttpResponse::Unauthorized().body("Invalid provider"),
    }
}

// Delete the user and the sessions linked to the user
// pub async fn delete_user(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
//     let user_table_result = sqlx::query!("DELETE FROM users WHERE user_id = $1", user_id.clone())
//         .execute(&**pool)
//         .await;
//
//     match user_table_result {
//         Ok(rows_affected) if rows_affected.rows_affected() > 0 => HttpResponse::Ok().finish(),
//         _ => HttpResponse::NotFound().finish(),
//     }
// }
