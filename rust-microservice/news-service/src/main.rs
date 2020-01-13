#[macro_use] extern crate log;
extern crate env_logger;
extern crate news_contract;
extern crate actix_web;

mod endpoints;
use endpoints::*;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    info!("Rust Actix Server running... http://localhost:8080/");
    HttpServer::new(|| App::new()
        .service(index)
        .service(list_news)
        .service(insert_news)
        .service(get_news_by_id)
        .service(delete_news_by_id)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}