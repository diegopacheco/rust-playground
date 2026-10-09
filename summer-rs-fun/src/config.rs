use serde::Deserialize;
use summer::config::Configurable;

#[derive(Debug, Configurable, Deserialize)]
#[config_prefix = "app-info"]
pub struct AppInfo {
    pub name: String,
    pub greeting: String,
}
