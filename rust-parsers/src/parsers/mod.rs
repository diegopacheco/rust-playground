pub mod maven;
pub mod sbt;
pub mod python;
pub mod npm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub group_id: String,
    pub artifact_id: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyDep {
    pub name: String,
    pub version: Option<String>,
}
