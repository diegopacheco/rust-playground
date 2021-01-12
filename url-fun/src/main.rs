use url::{Url, Host, Position};

fn main() {

    let data_url = Url::parse("data:text/plain,Hello?World#").unwrap();

    assert!(data_url.cannot_be_a_base());
    assert!(data_url.scheme() == "data");
    assert!(data_url.path() == "text/plain,Hello");
    assert!(data_url.path_segments().is_none());
    assert!(data_url.query() == Some("World"));
    assert!(data_url.fragment() == Some(""));
    println!("{}",data_url);
}
