extern crate news_contract;
extern crate tokio_postgres;
extern crate uuid;

use news_contract::News;
use tokio_postgres::{NoTls};
use tokio;

pub async fn insert_news(url:&String,desc:&String) -> Option<News> {
  Some(News{id:String::from("1"),desc:String::from("face"),url:String::from("fb.com")})
}

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
  let rows = &client.query("SELECT id::text,url,'desc' FROM news", &[]).await.unwrap();
  for row in rows {
    let news = News { 
        id:   row.get(0),
        desc: row.get(1),
        url:  row.get(2),
    };
    vec_news.push(news);
  }
  return Some(vec_news);
}

pub async fn mocked_list_news() -> Option<Vec<News>> {
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
}