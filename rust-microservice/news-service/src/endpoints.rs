use actix_web::{get,Responder,HttpResponse};
#[path="service.rs"] mod service;
use news_contract::News;

#[get("/")]
async fn index() -> impl Responder {
    format!("Endpoints: /news ")
}

#[get("/news")]
async fn list_news() -> HttpResponse  {
    let news:Vec<News> = service::list_news().unwrap();
    let json:String = serde_json::to_string(&news).unwrap();
    HttpResponse::Ok().json(json)
}