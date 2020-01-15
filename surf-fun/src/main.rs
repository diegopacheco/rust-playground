extern crate surf;
extern crate async_std;
extern crate serde_json;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut res = surf::get("https://httpbin.org/get").await?;
    let result = res.body_string().await?;
    println!("{}", serde_json::to_string_pretty(&result).unwrap() );
    Ok(())
}