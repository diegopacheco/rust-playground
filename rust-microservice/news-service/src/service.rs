extern crate news_contract;
extern crate news_dao;

use news_contract::News;

pub async fn get_news_by_id(id:&String) -> Option<News> {
  return news_dao::get_news_by_id(id).await;
}

pub async fn delete_news_by_id(id:&String) -> Option<bool> {
  return news_dao::delete_news_by_id(id).await;
}

pub async fn list_news() -> Option<Vec<News>> {
  return news_dao::list_news().await;
}

pub async fn insert_news(url:&String,desc:&String) -> Option<News> {
  return news_dao::insert_news(&url,&desc).await;
}