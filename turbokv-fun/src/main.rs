use std::path::PathBuf;

use turbokv_fun::{Result, Store};

#[tokio::main]
async fn main() -> Result<()> {
    let path = PathBuf::from("data");
    let store = Store::open(&path).await?;

    store.put("user:1", "Ada Lovelace").await?;
    store
        .put_all(&[
            ("user:2", "Grace Hopper"),
            ("user:3", "Alan Turing"),
            ("lang:1", "Rust"),
        ])
        .await?;

    println!("get user:1        -> {:?}", store.get("user:1").await?);
    println!("get user:404      -> {:?}", store.get("user:404").await?);
    println!("count user:       -> {}", store.count("user:").await?);

    for (key, value) in store.list("user:").await? {
        println!("scan user:        -> {key} = {value}");
    }

    store.remove("user:3").await?;
    println!("remove user:3     -> ok");
    println!("contains user:3   -> {}", store.contains("user:3").await?);
    println!("count user:       -> {}", store.count("user:").await?);

    store.flush().await?;
    store.close().await?;

    println!("database at {}", path.display());
    Ok(())
}
