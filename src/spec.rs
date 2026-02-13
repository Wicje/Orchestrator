use serde::{Deserialize, Serialize};

/// The input from the user – declarative and versioned.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSpec {
    pub spec_version: u32,
    pub language: String,
    pub framework: String,
    pub features: Vec<String>,
}

impl ProjectSpec {
    /// Load from a JSON file.
    pub fn from_json_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let spec: ProjectSpec = serde_json::from_str(&contents)?;
        Ok(spec)
    }
}
