use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{middleware::from_fn, web, App, HttpServer};
use dotenv::dotenv;
use handlers::jwt_middleware;
use routes::configure_routes;

mod config;
mod db;
mod handlers;
mod models;
mod routes;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    dotenv().ok();

    let pool = db::create_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(jwt_middleware))
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
            .configure(configure_routes)
    })
    .listen(listener)?
    .run()
    .await
}
