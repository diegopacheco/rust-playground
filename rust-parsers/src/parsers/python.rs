use anyhow::Result;
use std::fs;
use std::path::Path;

use super::PyDep;

pub fn parse_requirements(path: &Path) -> Result<Vec<PyDep>> {
    let content = fs::read_to_string(path)?;
    let mut out = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some((name, ver)) = split_name_version(line) {
            out.push(PyDep { name: name.to_string(), version: ver.map(|s| s.to_string()) });
        } else {
            out.push(PyDep { name: line.to_string(), version: None });
        }
    }
    Ok(out)
}

fn split_name_version(s: &str) -> Option<(&str, Option<&str>)> {
    let seps = ["==", ">=", "<=", "~=", "!=", ">", "<", "==="];
    for sep in seps {
        if let Some(idx) = s.find(sep) {
            let (name, rest) = s.split_at(idx);
            let rest = &rest[sep.len()..];
            let version = rest.split(|c: char| c.is_whitespace() || c == ';' || c == '#').next().filter(|v| !v.is_empty());
            return Some((name.trim(), version));
        }
    }
    None
}
