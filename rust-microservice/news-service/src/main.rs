#[macro_use] extern crate log;
extern crate env_logger;
mod endpoints;
use endpoints::*;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Rust Actix Server running... http://localhost:8080/");
    HttpServer::new(|| App::new()
        .service(index)
        .service(list_news))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}