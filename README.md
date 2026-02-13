Orchestrator

Orchestrator is a Rust-based CLI tool that reads a project specification file and automatically generates and executes a setup plan.

Instead of manually running scaffold commands and installing dependencies, you describe your project in a JSON spec file. Orchestrator builds a reproducible plan and applies it for you.

Features

Generate project setup plans from a JSON specification

Apply plans to scaffold new projects

Install dependencies and development tools automatically

Clean separation between specification, planning, and execution

Testable engine logic

Project Structure
src/
├── main.rs        # Entry point
├── cli.rs         # CLI argument parsing and command handling
├── spec.rs        # Spec file parsing and validation
├── registry/
│   ├── mod.rs     # Registry module root
│   ├── models.rs  # Framework & feature definitions
│   └── sqlite.rs  # SQLite-backed registry implementation
├── engine.rs      # Core logic that builds plans from specs
├── plan.rs        # Plan data structures
└── executor.rs    # Executes plans (scaffolding and installs)

Architecture Overview

Orchestrator is structured into clear layers:

Spec – Describes what the user wants.

Engine – Converts the spec into a concrete plan.

Plan – A structured set of executable steps.

Executor – Runs the steps defined in the plan.

Registry – Stores knowledge about frameworks and features.

This separation keeps the system modular, testable, and extensible.

Installation

Make sure you have Rust installed.

Then build the project:

cargo build --release


The compiled binary will be available at:

target/release/orchestrator

Running Tests
cargo test


All engine-related tests should pass successfully.

Usage
1. Create a Spec File

Example: myapp.json

{
  "spec_version": 1,
  "language": "javascript",
  "framework": "react",
  "features": ["tailwind", "eslint"]
}

2. Generate a Plan
./target/release/orchestrator plan myapp.json -o plan.json


This creates a plan.json file containing all setup steps.

3. Apply the Plan

Option A — Apply from generated plan:

mkdir testapp
cd testapp
../target/release/orchestrator apply ../plan.json


Option B — Apply directly from spec:

../target/release/orchestrator apply ../myapp.json --from-spec

What Happens Internally

For the example spec above, Orchestrator will:

Run Vite to scaffold a React project

Install TailwindCSS as a dev dependency

Install ESLint as a dev dependency

All of this is derived from the specification file.

Why This Exists

Orchestrator separates project intent from execution.

Instead of manually remembering setup commands, you declare what you want in a spec file. The engine converts that into a deterministic plan, and the executor applies it.

This makes project setup:

Reproducible

Automatable

Extendable

Easier to test

Future Improvements

Support additional languages and frameworks

Remote registry support

Plugin system for custom features

Plan diffing and dry-run mode
