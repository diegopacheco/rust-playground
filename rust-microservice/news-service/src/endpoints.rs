use actix_web::{get,Responder,HttpResponse};
#[path="service.rs"] mod service;

#[get("/")]
async fn index() -> impl Responder {
    format!("Endpoints: /news ")
}

#[get("/news")]
pub async fn list_news() -> HttpResponse  {
    let news = service::list_news().await;
    HttpResponse::Ok().json(news)
}