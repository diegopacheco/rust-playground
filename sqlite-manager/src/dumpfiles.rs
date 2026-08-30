use std::path::{Path, PathBuf};

use crate::error::DumpError;
use crate::manifest;

pub struct DumpFiles {
    pub schema: PathBuf,
    pub data: PathBuf,
    pub manifest: Option<PathBuf>,
}

impl DumpFiles {
    pub fn locate(directory: &Path) -> Result<Self, DumpError> {
        let manifest = directory.join(manifest::FILE);
        let files = Self {
            schema: directory.join("schema.sql"),
            data: directory.join("data.sql"),
            manifest: manifest.is_file().then_some(manifest),
        };
        if !files.schema.is_file() || !files.data.is_file() {
            return Err(DumpError::MissingDump(directory.to_path_buf()));
        }
        Ok(files)
    }
}
