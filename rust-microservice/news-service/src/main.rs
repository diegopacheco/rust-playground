use actix_web::{get, web, App, HttpServer, Responder,HttpResponse};
mod service;

#[get("/")]
async fn index() -> impl Responder {
    format!("Endpoints: /news ")
}

#[get("/news")]
async fn list_news() -> impl Responder {
    format!("{}",service::list_news().unwrap())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Rust Actix Server running... http://localhost:8080/");
    HttpServer::new(|| App::new()
        .service(index)
        .service(list_news))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}