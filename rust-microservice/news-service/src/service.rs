extern crate news_contract;
extern crate news_dao;
use news_contract::News;

pub fn list_news() -> Option<News> {
  return news_dao::list_news();
}
