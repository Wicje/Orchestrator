use crate::plan::ExecutionPlan;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Executor;

impl Executor {
    /// Execute the plan in the given target directory.
    pub fn execute(plan: &ExecutionPlan, target_dir: &Path) -> Result<()> {
        // Create target directory if it doesn't exist
        if !target_dir.exists() {
            fs::create_dir_all(target_dir)
                .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;
        }

        // 1. Run scaffold command (if any)
        if let Some(scaffold) = &plan.scaffold {
            println!("Running scaffold: {} {:?}", scaffold.command, scaffold.args);
            let status = Command::new(&scaffold.command)
                .args(&scaffold.args)
                .current_dir(target_dir)
                .status()
                .with_context(|| format!("Failed to execute scaffold command: {}", scaffold.command))?;
            if !status.success() {
                anyhow::bail!("Scaffold command exited with non-zero status");
            }
        }

        // 2. Write package.json or install dependencies (simplified)
        // For a real tool, you'd read existing package.json and merge dependencies.
        // Here we'll just run npm install with the dependencies.
        if !plan.dependencies.is_empty() || !plan.dev_dependencies.is_empty() {
            // Build npm install command
            let mut cmd = Command::new("npm");
            cmd.arg("install");
            for dep in &plan.dependencies {
                cmd.arg(format!("{}@{}", dep.name, dep.version));
            }
            if !plan.dev_dependencies.is_empty() {
                cmd.arg("--save-dev");
                for dep in &plan.dev_dependencies {
                    cmd.arg(format!("{}@{}", dep.name, dep.version));
                }
            }
            println!("Running: {:?}", cmd);
            let status = cmd
                .current_dir(target_dir)
                .status()
                .context("Failed to run npm install")?;
            if !status.success() {
                anyhow::bail!("npm install failed");
            }
        }

        // 3. Write files
        for file_write in &plan.file_writes {
            let path = target_dir.join(&file_write.path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, &file_write.content)
                .with_context(|| format!("Failed to write file: {}", path.display()))?;
        }

        // 4. Apply file modifications (very basic)
        for modification in &plan.file_modifications {
            let path = target_dir.join(&modification.path);
            if path.exists() {
                let mut content = fs::read_to_string(&path)?;
                // For demo, just append content (only handles text_append)
                if modification.mutation_type == "text_append" {
                    content.push_str(&modification.content);
                    fs::write(&path, content)?;
                } else {
                    println!("Skipping unknown mutation type: {}", modification.mutation_type);
                }
            } else {
                // If file doesn't exist, create it with the content
                fs::write(&path, &modification.content)?;
            }
        }

        // 5. Add scripts to package.json? Not implemented.

        Ok(())
    }
}
