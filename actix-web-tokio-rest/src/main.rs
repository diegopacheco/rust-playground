mod model;
use model::Cat;
mod state;
use state::AppState;
mod controller;
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use controller::{list_cats, add_cat};
use log::{info, LevelFilter};
use env_logger::Builder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    Builder::new()
    .format_timestamp_secs()
    .filter_level(LevelFilter::Info)
    .init();

    let app_state = web::Data::new(AppState {
        cats: Mutex::new(vec![
            Cat {
                name: "Bobcat".to_string(),
                cat_type: "wild".to_string(),
                fun_fact: "hunt bugs at night".to_string(),
            }
        ]),
    });

    info!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(list_cats)
            .service(add_cat)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_list_cats() {
        let app_state = web::Data::new(AppState {
            cats: Mutex::new(vec![
                Cat {
                    name: "TestCat".to_string(),
                    cat_type: "test".to_string(),
                    fun_fact: "loves testing".to_string(),
                }
            ]),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(list_cats)
        ).await;

        let req = test::TestRequest::get().uri("/cats").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_add_cat() {
        let app_state = web::Data::new(AppState {
            cats: Mutex::new(vec![]),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(add_cat)
        ).await;

        let cat = Cat {
            name: "TestCat".to_string(),
            cat_type: "test".to_string(),
            fun_fact: "loves testing".to_string(),
        };

        let req = test::TestRequest::put()
            .uri("/cats")
            .set_json(&cat)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let cats = app_state.cats.lock().unwrap();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].name, "TestCat");
    }
}