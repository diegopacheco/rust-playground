use actix_web::{get,put,delete,Responder,HttpResponse,web};
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

#[get("/news/{id}")]
pub async fn get_news_by_id(info:web::Path<String>) -> HttpResponse  {
    let id  = &info.as_str();
    let mut new_string = String::new();
    new_string.push_str(id);

    let news = service::get_news_by_id(&new_string).await;
    HttpResponse::Ok().json(news)
}

#[delete("/news/{id}")]
pub async fn delete_news_by_id(info:web::Path<String>) -> HttpResponse {
    let id  = &info.as_str();
    let mut new_string = String::new();
    new_string.push_str(id);

    let news = service::delete_news_by_id(&new_string).await;
    HttpResponse::Ok().json(news)
}

#[put("/news/{url}/{desc}")]
pub async fn insert_news(info:web::Path<(String, String)>) -> impl Responder {
    let url  = &info.0;
    let desc = &info.1;
    let new = service::insert_news(url,desc).await;
    HttpResponse::Ok().json(new)
}