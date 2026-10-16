use std::time::Duration;

use tokio::net::TcpListener;

mod books;
mod components;
mod layout;
mod routes;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    println!("Proa Bookshelf listening on http://localhost:{port}");
    proa_framework_axum::serve(listener, routes::router(), Duration::from_secs(5)).await?;
    Ok(())
}
