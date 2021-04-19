use salvo::prelude::*;

#[fn_handler]
async fn hello_world() -> &'static str {
    "Hello World"
}
#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);
    let server = Server::new(router);
    println!("Goto http://127.0.0.1:7878");
    server.bind(([0, 0, 0, 0], 7878)).await;
}