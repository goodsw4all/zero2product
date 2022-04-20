use actix_web::dev::Server;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// #[get("/myservice")]
// async fn myservice() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

pub fn run() -> Result<Server, std::io::Error> {
    // HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
    //     .bind("127.0.0.1:8000")?
    //     .run();
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
