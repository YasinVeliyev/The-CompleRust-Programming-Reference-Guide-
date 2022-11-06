use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use log::info;
use std::env;

mod route_handlers;
mod state;

use route_handlers::*;

fn init_env() {
    env::set_var("RUST_LOG", "linksnap=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    info!("Starting http server: 127.0.0.1:8080");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();
    // let system = actix::System::new();
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
