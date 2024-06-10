use actix_web::{App, get, HttpResponse, HttpServer, Responder};

use crate::filesystem::initialize_file_structure;
use crate::logging::{info};

mod logging;
mod api;
mod filesystem;

const PORT: u16 = 8765;
const APP_NAME: &str = "Broodkruimel";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    initialize_file_structure().await;
    info(format!("Server up at http://127.0.0.1:{}", PORT), Some(APP_NAME));
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(favicon)
            .service(ping)
            .service(api::user::create_user)
            .service(api::user::get_user)
            .service(api::geospatial::push_location)
    })
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("<img src='https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExN2JqN3c1NjFlcDczcGZpdG13YzlsandlOW1qNnNvdWI3NHYxa2hjZCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/felYMe70wGGCc1YtXE/giphy.gif' alt='Broodkruimel'>")
}

/// Troll browser
#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    HttpResponse::ImATeapot()
}

#[get("/ping")]
async fn ping() -> impl Responder {
    info("Pong!", Some("Ping"));
    HttpResponse::Ok().body("<h1>taco</h1>")
}