use std::path::{Path, PathBuf};

use crate::error::DumpError;

pub struct DumpFiles {
    pub schema: PathBuf,
    pub data: PathBuf,
}

impl DumpFiles {
    pub fn locate(directory: &Path) -> Result<Self, DumpError> {
        let files = Self {
            schema: directory.join("schema.sql"),
            data: directory.join("data.sql"),
        };
        if !files.schema.is_file() || !files.data.is_file() {
            return Err(DumpError::MissingDump(directory.to_path_buf()));
        }
        Ok(files)
    }
}
