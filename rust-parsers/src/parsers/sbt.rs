use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

use super::Artifact;

#[derive(Debug, Clone)]
pub struct SbtOut {
    pub dependencies: Vec<Artifact>,
    pub plugins: Vec<Artifact>,
}

pub fn parse_build_sbt(path: &Path) -> Result<SbtOut> {
    let content = fs::read_to_string(path)?;
    let mut deps = Vec::new();
    for line in content.lines() {
        let l = line.trim();
        if l.is_empty() { continue; }
        if l.contains("addSbtPlugin") { continue; }
        if let Some(a) = parse_dep_line(l) {
            deps.push(a);
        }
    }

    let plugins_re = Regex::new(r#"\s*addSbtPlugin\(\s*"([^"]+)"\s*%\s*"([^"]+)"\s*%\s*"([^"]+)"\s*\)"#).unwrap();
    let mut plugins = Vec::new();
    for c in plugins_re.captures_iter(&content) {
        let g = c.get(1).unwrap().as_str().to_string();
        let a = c.get(2).unwrap().as_str().to_string();
        let v = c.get(3).unwrap().as_str().to_string();
        plugins.push(Artifact { group_id: g, artifact_id: a, version: Some(v) });
    }

    Ok(SbtOut { dependencies: deps, plugins })
}

fn parse_dep_line(line: &str) -> Option<Artifact> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() { i += 1; }
    if i >= bytes.len() || bytes[i] != b'"' { return None; }
    i += 1;
    let start_g = i;
    while i < bytes.len() && bytes[i] != b'"' { i += 1; }
    if i >= bytes.len() { return None; }
    let group = &line[start_g..i];
    i += 1;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() { i += 1; }
    if i >= bytes.len() || bytes[i] != b'%' { return None; }
    i += 1;
    if i < bytes.len() && bytes[i] == b'%' { i += 1; }
    while i < bytes.len() && bytes[i].is_ascii_whitespace() { i += 1; }
    if i >= bytes.len() || bytes[i] != b'"' { return None; }
    i += 1;
    let start_a = i;
    while i < bytes.len() && bytes[i] != b'"' { i += 1; }
    if i >= bytes.len() { return None; }
    let artifact = &line[start_a..i];
    i += 1;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() { i += 1; }
    if i >= bytes.len() || bytes[i] != b'%' { return None; }
    i += 1;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() { i += 1; }
    if i >= bytes.len() || bytes[i] != b'"' { return None; }
    i += 1;
    let start_v = i;
    while i < bytes.len() && bytes[i] != b'"' { i += 1; }
    if i > line.len() { return None; }
    let version = &line[start_v..i];
    Some(Artifact { group_id: group.to_string(), artifact_id: artifact.to_string(), version: Some(version.to_string()) })
}
