use crate::routes::health_check;
use crate::routes::subscribe;

use actix_web::dev::Server;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hi, I'm here")
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
