extern crate news_contract;
extern crate tokio_postgres;
extern crate uuid;

use news_contract::News;
use tokio_postgres::{NoTls};
use tokio;

pub async fn connect() -> Option<tokio_postgres::Client> {
  let (client,conn) =
        tokio_postgres::connect("host=172.17.0.2 user=postgres password=docker dbname=postgres port=5432", NoTls).
        await.unwrap();
  tokio::spawn(async move {
      if let Err(e) = conn.await {
          eprintln!("connection error: {}", e);
      }
  });
  return Some(client);
}

pub async fn get_news_by_id(id:&String) -> Option<News> {
  let client = connect().await.unwrap();
  let rows = &client.query("SELECT id::text,url,'desc' FROM news where id::text=$1", &[&id]).await.unwrap();
  let row = rows.get(0).unwrap();
    let news = News { 
        id:   row.get(0),
        desc: row.get(2),
        url:  row.get(1),
    };
  return Some(news);
}

pub async fn delete_news_by_id(id:&String) -> Option<bool> {
  let client = connect().await.unwrap();
  let _rows = &client.query("DELETE FROM news where id::text=$1", &[&id]).await.unwrap();
  return Some(true);
}

pub async fn insert_news(url:&String,desc:&String) -> Option<News> {
  let client = connect().await.unwrap();
  let _row = client.query("INSERT INTO news VALUES(uuid_in(md5(random()::text || clock_timestamp()::text)::cstring),$1,$2)",&[&desc,&url]).await.unwrap();
  let news = News {
    id: String::from("0"),
    desc: String::from(desc),
    url: String::from(url),
  };
  return Some(news);
}

pub async fn list_news() -> Option<Vec<News>> {
  let client = connect().await.unwrap();
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