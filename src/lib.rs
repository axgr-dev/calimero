use actix_web::dev::Server;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health))
        .listen(listener)?
        .run();

    Ok(server)
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
