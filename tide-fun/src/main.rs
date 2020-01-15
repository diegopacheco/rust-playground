extern crate tide;
extern crate async_std;

#[async_std::main]
async fn main() {
    let mut app = tide::new();
    app.at("/").get(|_| async move { "Hello, world!" });
    app.listen("127.0.0.1:8080").await;
}