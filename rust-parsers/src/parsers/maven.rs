use anyhow::{anyhow, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;
use std::path::Path;

use super::Artifact;

#[derive(Debug, Clone)]
pub struct PomOut {
    pub dependencies: Vec<Artifact>,
    pub plugins: Vec<Artifact>,
}

pub fn parse_pom(path: &Path) -> Result<PomOut> {
    let xml = fs::read_to_string(path)?;
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut stack: Vec<String> = Vec::new();
    let mut cur_group: Option<String> = None;
    let mut cur_artifact: Option<String> = None;
    let mut cur_version: Option<String> = None;
    let mut in_dependency = false;
    let mut in_plugin = false;

    let mut deps = Vec::new();
    let mut plugins = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                stack.push(name.clone());
                if stack.ends_with(&["project".into(), "dependencies".into(), "dependency".into()])
                    || stack.ends_with(&["dependencies".into(), "dependency".into()])
                {
                    in_dependency = true;
                    cur_group = None;
                    cur_artifact = None;
                    cur_version = None;
                }
                if stack.ends_with(&["project".into(), "build".into(), "plugins".into(), "plugin".into()])
                    || stack.ends_with(&["plugins".into(), "plugin".into()])
                {
                    in_plugin = true;
                    cur_group = None;
                    cur_artifact = None;
                    cur_version = None;
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if let Some(last) = stack.last() {
                    match last.as_str() {
                        "groupId" => {
                            if in_dependency || in_plugin {
                                cur_group = Some(text);
                            }
                        }
                        "artifactId" => {
                            if in_dependency || in_plugin {
                                cur_artifact = Some(text);
                            }
                        }
                        "version" => {
                            if in_dependency || in_plugin {
                                cur_version = Some(text);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "dependency" && in_dependency {
                    if let (Some(g), Some(a)) = (cur_group.clone(), cur_artifact.clone()) {
                        deps.push(Artifact { group_id: g, artifact_id: a, version: cur_version.clone() });
                    }
                    in_dependency = false;
                } else if name == "plugin" && in_plugin {
                    if let (Some(g), Some(a)) = (cur_group.clone(), cur_artifact.clone()) {
                        plugins.push(Artifact { group_id: g, artifact_id: a, version: cur_version.clone() });
                    }
                    in_plugin = false;
                }
                stack.pop();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow!(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(PomOut { dependencies: deps, plugins })
}
