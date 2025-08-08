use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct PackageJsonRaw {
    #[serde(default)]
    dependencies: BTreeMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    dev_dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct NpmOut {
    pub dependencies: BTreeMap<String, String>,
    pub dev_dependencies: BTreeMap<String, String>,
}

pub fn parse_package_json(path: &Path) -> Result<NpmOut> {
    let s = fs::read_to_string(path)?;
    let raw: PackageJsonRaw = serde_json::from_str(&s)?;
    Ok(NpmOut { dependencies: raw.dependencies, dev_dependencies: raw.dev_dependencies })
}
