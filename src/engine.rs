use crate::plan::*;
use crate::registry::Registry;
use crate::spec::ProjectSpec;
use anyhow::{bail, Result};

pub struct Engine;

impl Engine {
    /// Validate the spec against the registry, resolve dependencies and generate a plan.
    pub fn resolve(spec: &ProjectSpec, registry: &dyn Registry) -> Result<ExecutionPlan> {
        // 1. Validate language support
        if !registry.framework_supports_language(&spec.framework, &spec.language)? {
            bail!(
                "Framework '{}' does not support language '{}'",
                spec.framework,
                spec.language
            );
        }

        // 2. Validate each feature is compatible with the framework
        for feature in &spec.features {
            if !registry.is_feature_compatible(&spec.framework, feature)? {
                bail!(
                    "Feature '{}' is not compatible with framework '{}'",
                    feature,
                    spec.framework
                );
            }
        }

        // 3. Gather dependencies (global and framework-specific)
        let deps = registry.get_dependencies(Some(&spec.framework), &spec.features)?;

        // Separate into normal and dev dependencies
        let mut dependencies = Vec::new();
        let mut dev_dependencies = Vec::new();
        for d in deps {
            let dep = DependencyInstall {
                name: d.package_name,
                version: d.version_constraint,
            };
            if d.is_dev {
                dev_dependencies.push(dep);
            } else {
                dependencies.push(dep);
            }
        }

        // 4. Get config mutations
        let mutations = registry.get_config_mutations(&spec.framework, &spec.features)?;
        let file_modifications = mutations
            .into_iter()
            .map(|m| FileModification {
                path: m.file_path,
                mutation_type: m.mutation_type,
                content: m.content,
            })
            .collect();

        // 5. Determine scaffold step (if any)
        let scaffold = if let Some(cmd) = registry.get_scaffold_command(&spec.framework)? {
            // For now we assume the command is a single string; we might need to parse args.
            // Simple split by spaces is naive but works for our examples.
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
                None
            } else {
                Some(ScaffoldStep {
                    command: parts[0].to_string(),
                    args: parts[1..].iter().map(|s| s.to_string()).collect(),
                })
            }
        } else {
            None
        };

        // 6. Build the final plan (file_writes and scripts are empty for now)
        Ok(ExecutionPlan {
            scaffold,
            dependencies,
            dev_dependencies,
            file_writes: vec![], // could be filled from templates later
            file_modifications,
            scripts: vec![], // could be filled from package.json scripts
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::sqlite::SqliteRegistry;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_react_typescript() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let registry = SqliteRegistry::new(&db_path).unwrap();

        let spec = ProjectSpec {
            spec_version: 1,
            language: "javascript".to_string(),
            framework: "react".to_string(),
            features: vec!["typescript".to_string()],
        };

        let plan = Engine::resolve(&spec, &registry).unwrap();
        assert!(plan.scaffold.is_some());
        assert_eq!(plan.dependencies.len(), 0); // react and react-dom are in scaffold? Not in our data; we only have feature deps.
        // In our seed, typescript is dev dep only.
        assert_eq!(plan.dev_dependencies.len(), 1);
        assert_eq!(plan.dev_dependencies[0].name, "typescript");
    }

    #[test]
    fn test_invalid_language() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let registry = SqliteRegistry::new(&db_path).unwrap();

        let spec = ProjectSpec {
            spec_version: 1,
            language: "python".to_string(),
            framework: "react".to_string(),
            features: vec![],
        };

        let result = Engine::resolve(&spec, &registry);
        assert!(result.is_err());
    }
}
