use reqwest::Client;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    print!(">>> HTTP Call\n");

    let c = Client::new();
    let res = c.get("https://httpbin.org/get").send().await?;
    let status = res.status();
    let text = res.text().await?;
    
    println!("{}", status);
    println!("{}", text.len());
    Ok(())
}
