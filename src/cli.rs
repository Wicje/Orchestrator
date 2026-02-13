use crate::engine::Engine;
use crate::executor::Executor;
use crate::registry::sqlite::SqliteRegistry;
use crate::spec::ProjectSpec;
use anyhow::Result;
use clap::{Parser, Subcommand};
use dirs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Deterministic project bootstrapping engine")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate an execution plan from a spec file
    Plan {
        /// Path to the project spec JSON file
        spec: PathBuf,
        /// Output file for the plan (default: plan.json in current dir)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Execute a plan (or a spec directly) in a target directory
    Apply {
        /// Path to the plan JSON file or spec file
        input: PathBuf,
        /// Target directory (defaults to current directory)
        #[arg(short, long)]
        target: Option<PathBuf>,
        /// If input is a spec, resolve and execute directly
        #[arg(long)]
        from_spec: bool,
    },
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Cli::parse();

        // Determine database path (store in user's config directory)
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("orchestrator");
        std::fs::create_dir_all(&config_dir)?;
        let db_path = config_dir.join("registry.db");

        let registry = SqliteRegistry::new(&db_path)?;

        match cli.command {
            Commands::Plan { spec, output } => {
                let spec = ProjectSpec::from_json_file(&spec)?;
                let plan = Engine::resolve(&spec, &registry)?;
                let out_path = output.unwrap_or_else(|| PathBuf::from("plan.json"));
                let plan_json = serde_json::to_string_pretty(&plan)?;
                std::fs::write(out_path, plan_json)?;
                println!("Plan written to {:?}", out_path);
            }
            Commands::Apply { input, target, from_spec } => {
                let target_dir = target.unwrap_or_else(|| PathBuf::from("."));
                if from_spec {
                    // Interpret input as a spec file
                    let spec = ProjectSpec::from_json_file(&input)?;
                    let plan = Engine::resolve(&spec, &registry)?;
                    Executor::execute(&plan, &target_dir)?;
                    println!("Project generated in {}", target_dir.display());
                } else {
                    // Interpret input as a plan file
                    let plan_json = std::fs::read_to_string(&input)?;
                    let plan: crate::plan::ExecutionPlan = serde_json::from_str(&plan_json)?;
                    Executor::execute(&plan, &target_dir)?;
                    println!("Plan executed in {}", target_dir.display());
                }
            }
        }
        Ok(())
    }
}
