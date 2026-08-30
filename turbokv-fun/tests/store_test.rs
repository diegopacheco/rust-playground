use std::fs;
use std::path::PathBuf;

use turbokv_fun::{Result, Store};

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("turbokv-fun-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        Self { path }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[tokio::test]
async fn get_returns_the_last_written_value() -> Result<()> {
    let dir = TempDir::new("overwrite");
    let store = Store::open(&dir.path).await?;

    store.put("user:1", "Ada").await?;
    store.put("user:1", "Ada Lovelace").await?;

    assert_eq!(store.get("user:1").await?, Some("Ada Lovelace".to_string()));
    store.close().await
}

#[tokio::test]
async fn missing_key_is_none_and_empty_value_is_some() -> Result<()> {
    let dir = TempDir::new("empty");
    let store = Store::open(&dir.path).await?;

    store.put("user:1", "").await?;

    assert_eq!(store.get("user:1").await?, Some(String::new()));
    assert_eq!(store.get("user:404").await?, None);
    store.close().await
}

#[tokio::test]
async fn batch_publishes_every_entry() -> Result<()> {
    let dir = TempDir::new("batch");
    let store = Store::open(&dir.path).await?;

    store
        .put_all(&[("user:1", "Ada"), ("user:2", "Grace"), ("user:3", "Alan")])
        .await?;

    assert_eq!(store.count("user:").await?, 3);
    assert_eq!(store.get("user:2").await?, Some("Grace".to_string()));
    store.close().await
}

#[tokio::test]
async fn scan_is_limited_to_the_prefix_and_ordered_by_key() -> Result<()> {
    let dir = TempDir::new("prefix");
    let store = Store::open(&dir.path).await?;

    store
        .put_all(&[
            ("user:3", "Alan"),
            ("lang:1", "Rust"),
            ("user:1", "Ada"),
            ("user:2", "Grace"),
        ])
        .await?;

    let users = store.list("user:").await?;

    assert_eq!(
        users,
        vec![
            ("user:1".to_string(), "Ada".to_string()),
            ("user:2".to_string(), "Grace".to_string()),
            ("user:3".to_string(), "Alan".to_string()),
        ]
    );
    store.close().await
}

#[tokio::test]
async fn removed_key_disappears_from_reads_and_scans() -> Result<()> {
    let dir = TempDir::new("remove");
    let store = Store::open(&dir.path).await?;

    store.put_all(&[("user:1", "Ada"), ("user:2", "Grace")]).await?;
    store.remove("user:1").await?;

    assert_eq!(store.get("user:1").await?, None);
    assert!(!store.contains("user:1").await?);
    assert_eq!(store.count("user:").await?, 1);
    store.close().await
}

#[tokio::test]
async fn data_survives_a_close_and_reopen() -> Result<()> {
    let dir = TempDir::new("persistence");

    let store = Store::open(&dir.path).await?;
    store.put_all(&[("user:1", "Ada"), ("user:2", "Grace")]).await?;
    store.remove("user:2").await?;
    store.close().await?;

    let store = Store::open(&dir.path).await?;
    assert_eq!(store.get("user:1").await?, Some("Ada".to_string()));
    assert_eq!(store.get("user:2").await?, None);
    store.close().await
}
