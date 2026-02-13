use anyhow::Result;
use std::sync::Arc;

pub mod models;
pub mod sqlite;

use models::*;

/// The registry knows everything about frameworks and features.
pub trait Registry: Send + Sync {
    /// Check if a framework supports a given language.
    fn framework_supports_language(&self, framework_id: &str, language: &str) -> Result<bool>;

    /// Get the base scaffold command for a framework (if any).
    fn get_scaffold_command(&self, framework_id: &str) -> Result<Option<String>>;

    /// List all features that can be added to a framework.
    fn features_for_framework(&self, framework_id: &str) -> Result<Vec<Feature>>;

    /// Check if a feature is compatible with a framework.
    fn is_feature_compatible(&self, framework_id: &str, feature_id: &str) -> Result<bool>;

    /// Get all dependencies required for a set of features (optionally framework-specific).
    fn get_dependencies(
        &self,
        framework_id: Option<&str>,
        features: &[String],
    ) -> Result<Vec<Dependency>>;

    /// Get config mutations for a framework + features.
    fn get_config_mutations(
        &self,
        framework_id: &str,
        features: &[String],
    ) -> Result<Vec<ConfigMutation>>;
}
