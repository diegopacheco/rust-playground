mod parsers;

use anyhow::Result;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let base = PathBuf::from("testdata");

    let pom = base.join("pom.xml");
    let sbt = base.join("build.sbt");
    let req = base.join("requirements.txt");
    let pkg = base.join("package.json");

    if pom.exists() {
        let out = parsers::maven::parse_pom(&pom)?;
        println!("pom.xml dependencies:");
        for d in &out.dependencies {
            println!("{},{},{}", d.group_id, d.artifact_id, d.version.clone().unwrap_or_default());
        }
        println!("pom.xml plugins:");
        for p in &out.plugins {
            println!("{},{},{}", p.group_id, p.artifact_id, p.version.clone().unwrap_or_default());
        }
    }

    if sbt.exists() {
        let out = parsers::sbt::parse_build_sbt(&sbt)?;
        println!("build.sbt dependencies:");
        for d in &out.dependencies {
            println!("{},{},{}", d.group_id, d.artifact_id, d.version.clone().unwrap_or_default());
        }
        println!("build.sbt plugins:");
        for p in &out.plugins {
            println!("{},{},{}", p.group_id, p.artifact_id, p.version.clone().unwrap_or_default());
        }
    }

    if req.exists() {
        let deps = parsers::python::parse_requirements(&req)?;
        println!("requirements.txt:");
        for d in &deps {
            println!("{},{}", d.name, d.version.clone().unwrap_or_default());
        }
    }

    if pkg.exists() {
        let deps = parsers::npm::parse_package_json(&pkg)?;
        println!("package.json dependencies:");
        for (n, v) in &deps.dependencies {
            println!("{},{}", n, v);
        }
        println!("package.json devDependencies:");
        for (n, v) in &deps.dev_dependencies {
            println!("{},{}", n, v);
        }
    }

    Ok(())
}
