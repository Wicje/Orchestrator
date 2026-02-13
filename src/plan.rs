use serde::{Deserialize, Serialize};

/// The output of the engine – a complete, deterministic plan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecutionPlan {
    pub scaffold: Option<ScaffoldStep>,
    pub dependencies: Vec<DependencyInstall>,
    pub dev_dependencies: Vec<DependencyInstall>,
    pub file_writes: Vec<FileWrite>,
    pub file_modifications: Vec<FileModification>,
    pub scripts: Vec<Script>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScaffoldStep {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DependencyInstall {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileWrite {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileModification {
    pub path: String,
    pub mutation_type: String, // e.g., "json_merge", "text_append"
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Script {
    pub name: String,
    pub command: String,
}
