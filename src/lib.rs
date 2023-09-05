use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health).service(add_contact))
        .listen(listener)?
        .run();

    Ok(server)
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    email: String,
    first_name: String,
    last_name: String,
}

#[post("/contacts")]
async fn add_contact(contact: web::Json<Contact>) -> HttpResponse {
    println!("Contact: {:?}", contact);
    HttpResponse::Ok().json(contact)
}
