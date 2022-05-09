use crate::routes::health_check;
use crate::routes::subscribe;

use actix_web::dev::Server;
// use actix_web::middleware::Logger;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};

use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use std::net::TcpListener;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hi, I'm here")
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = web::Data::new(db_pool);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            // .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
