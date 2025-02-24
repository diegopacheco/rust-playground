use actix_web::{get, put, web, HttpResponse, Responder};
use crate::state::AppState;
use crate::model::Cat;

#[get("/cats")]
pub async fn list_cats(data: web::Data<AppState>) -> impl Responder {
    let cats = data.cats.lock().unwrap();
    HttpResponse::Ok().json(&*cats)
}

#[put("/cats")]
pub async fn add_cat(cat: web::Json<Cat>, data: web::Data<AppState>) -> impl Responder {
    let mut cats = data.cats.lock().unwrap();
    cats.push(cat.into_inner());
    HttpResponse::Created().json(&*cats)
}