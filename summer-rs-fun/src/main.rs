mod config;
mod routes;
mod task;

use summer::{App, auto_config};
use summer_web::{WebConfigurator, WebPlugin};

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new().add_plugin(WebPlugin).run().await
}
