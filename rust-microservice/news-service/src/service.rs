extern crate news_contract;
use news_contract::News;

pub fn list_news() -> Option<News> {
    let news = News { id: 1, 
                      desc: String::from("rust kick add"),
                      url: String::from("http://localhost:8080/news"),
                    };
    return Some(news);
}
