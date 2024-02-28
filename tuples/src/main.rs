
fn get_server_config() -> Result<(String,i32),String> {
    let tuple = ("127.0.0.1".to_string(),8080);
    Ok(tuple)
}

fn main() {
    let config = get_server_config();
    println!("Config is {:?}", config);
}
