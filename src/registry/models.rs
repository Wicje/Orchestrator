use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Framework {
    pub id: String,
    pub language: String,
    pub base_scaffold_command: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Feature {
    pub id: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub package_name: String,
    pub version_constraint: String,
    pub is_dev: bool,
}

#[derive(Debug, Clone)]
pub struct ConfigMutation {
    pub file_path: String,
    pub mutation_type: String,
    pub content: String,
}
