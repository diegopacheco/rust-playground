use obsidian::{App, context::Context};

#[tokio::main]
async fn main() {
    let mut app: App = App::new();
    let address = ([127, 0, 0, 1], 3000).into();

    app.get("/", |ctx: Context| async { ctx.build("Hello World").ok() });
    app.listen(&address, || {
        println!("server is listening to {}", &address);
    }).await;
}
