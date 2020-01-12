extern crate news_contract;
extern crate tokio_postgres;
extern crate uuid;

use news_contract::News;
use tokio_postgres::{NoTls};
use tokio;
use uuid::Uuid;

pub async fn list_news() -> Option<Vec<News>> {

  let (client,conn) =
        tokio_postgres::connect("host=172.17.0.2 user=postgres password=docker dbname=postgres port=5432", NoTls).
        await.unwrap();
  
  tokio::spawn(async move {
      if let Err(e) = conn.await {
          eprintln!("connection error: {}", e);
      }
  });      

  let mut vec_news = Vec::new();  
  let rows = &client.query("SELECT * FROM news", &[]).await.unwrap();
  for row in rows {
    let news = News {
        id:   Uuid::parse_str(row.get(0)).unwrap().to_hyphenated().to_string(),
        desc: row.get(1),
        url:  row.get(2),
    };
    vec_news.push(news);
  }
  return Some(vec_news);
  /*
  let mut vec_news = Vec::new();  
  vec_news.push(News {
    id: String::from("1234"),
    desc: String::from("google"),
    url: String::from("google.com")
  });
  vec_news.push(News {
    id: String::from("1234"),
    desc: String::from("google"),
    url: String::from("google.com")
  });
  return Some(vec_news);
  */
}