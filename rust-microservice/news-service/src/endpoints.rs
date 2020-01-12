use actix_web::{get,Responder};
#[path="service.rs"] mod service;

#[get("/")]
async fn index() -> impl Responder {
    format!("Endpoints: /news ")
}

#[get("/news")]
async fn list_news() -> impl Responder {
    format!("{}",service::list_news().unwrap())
}