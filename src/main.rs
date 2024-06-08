use actix_web::{App, get, HttpResponse, HttpServer, Responder};

use crate::filesystem::initialize_file_structure;
use crate::logging::info;

mod logging;
mod api;
mod filesystem;

const PORT: u16 = 8080;
const APP_NAME: &str = "gpslog";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // to_gpx("test.gpx").unwrap();
    initialize_file_structure().await;
    info(format!("Server up at http://127.0.0.1:{}", PORT), Some(APP_NAME));
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(favicon)
            .service(api::user::create_user)
            .service(api::user::get_user)
            .service(api::location::add_location_to_user)
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