use std::io::Write;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use crate::filesystem::initialize_file_structure;

use crate::logging::info;

mod logging;
mod filesystem;
mod api;

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize_file_structure().await;
    info(format!("Server up at http://127.0.0.1:{}", PORT).as_str(), None);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(favicon)
            .service(api::user::create_user)
            .service(api::user::get_user)
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