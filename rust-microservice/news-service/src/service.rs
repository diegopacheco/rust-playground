extern crate news_contract;
extern crate news_dao;

use news_contract::News;

pub async fn get_news_by_id(uuid:&String) -> Option<News> {
  return news_dao::get_news_by_id(uuid).await;
}

pub async fn list_news() -> Option<Vec<News>> {
  return news_dao::list_news().await;
}

pub async fn insert_news(url:&String,desc:&String) -> Option<News> {
  return news_dao::insert_news(&url,&desc).await;
}