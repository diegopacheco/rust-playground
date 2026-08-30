use std::path::Path;

use turbokv::{Db, DbError, DbOptions, WriteBatch};

pub type Result<T> = std::result::Result<T, DbError>;

pub struct Store {
    db: Db,
}

impl Store {
    pub async fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = Db::open_with_options(path, DbOptions::durable()).await?;
        Ok(Self { db })
    }

    pub async fn put(&self, key: &str, value: &str) -> Result<()> {
        self.db.insert(key.as_bytes(), value.as_bytes()).await
    }

    pub async fn put_all(&self, entries: &[(&str, &str)]) -> Result<()> {
        let mut batch = WriteBatch::with_capacity(entries.len());
        for (key, value) in entries {
            batch.put(key.as_bytes(), value.as_bytes());
        }
        self.db.write_batch(&batch).await
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        Ok(self.db.get(key.as_bytes()).await?.map(decode))
    }

    pub async fn remove(&self, key: &str) -> Result<()> {
        self.db.remove(key.as_bytes()).await
    }

    pub async fn contains(&self, key: &str) -> Result<bool> {
        self.db.contains_key(key.as_bytes()).await
    }

    pub async fn list(&self, prefix: &str) -> Result<Vec<(String, String)>> {
        let entries = self.db.scan_prefix(prefix.as_bytes()).await?;
        Ok(entries
            .into_iter()
            .map(|(key, value)| (decode(key), decode(value)))
            .collect())
    }

    pub async fn count(&self, prefix: &str) -> Result<usize> {
        Ok(self.db.scan_prefix(prefix.as_bytes()).await?.len())
    }

    pub async fn flush(&self) -> Result<()> {
        self.db.flush().await
    }

    pub async fn close(self) -> Result<()> {
        self.db.close().await
    }
}

fn decode(bytes: Vec<u8>) -> String {
    String::from_utf8_lossy(&bytes).into_owned()
}
