use std::path::{Path, PathBuf};

use crate::error::DumpError;

pub struct Workspace {
    directory: PathBuf,
}

impl Workspace {
    pub fn create() -> Result<Self, DumpError> {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|elapsed| elapsed.as_nanos())
            .unwrap_or_default();
        let directory =
            std::env::temp_dir().join(format!("sqlite-manager-{}-{stamp}", std::process::id()));
        std::fs::create_dir_all(&directory).map_err(|e| DumpError::io(&directory, e))?;
        Ok(Self { directory })
    }

    pub fn database(&self) -> PathBuf {
        self.directory.join("workspace.db")
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

pub fn sibling(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.to_path_buf().into_os_string();
    name.push(suffix);
    PathBuf::from(name)
}
