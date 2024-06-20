use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use clap::Parser;

use crate::filesystem::initialize_file_structure;
use crate::logging::info;

mod logging;
mod api;
mod filesystem;

const APP_NAME: &str = "Broodkruimel";

#[derive(Parser, Debug)]
#[command(version, about = "Broodkruimel is a server meant for storing \"breadcrumbs\".\nThe idea is that clients send a live GPS-location every x seconds, and the users can view and analyze that collected data later on.")]
struct Args {
    /// Override the default port.
    #[clap(short, long, default_value = "8765")]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    initialize_file_structure().await;
    info(format!("Server up at http://0.0.0.0:{}", args.port), Some(APP_NAME));
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(favicon)
            .service(ping)
            .service(api::user::create_user)
            .service(api::user::get_users)
            .service(api::geospatial::push_location)
    })
        .bind(("0.0.0.0", args.port))?
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
    HttpResponse::Ok().body("pong!")
}