use std::io::Write;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};

use crate::logging::info;

mod user;
mod logging;

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info(format!("Server up at http://0.0.0.0:{}", PORT).as_str(), None);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(favicon)
            .service(user::create_user)
            .service(user::get_user)
    })
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// Troll browser
#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    HttpResponse::ImATeapot()
}