# Orchestrator

A deterministic project bootstrapping engine.  
You declare what you want (language, framework, features) – Orchestrator figures out how to build it.

Orchestrator is a Rust-based CLI tool that reads a project specification file and automatically generates and executes a setup plan. Instead of manually running scaffold commands and installing dependencies, you describe your project in a JSON spec file. Orchestrator builds a reproducible plan and applies it for you.

# Example: generate a React + Tailwind + ESLint project
```bash
orchestrator plan myapp.json -o plan.json
orchestrator apply plan.json```

## Features
Generate project setup plans from a JSON specification

Apply plans to scaffold new projects

Install dependencies and development tools automatically

Clean separation between specification, planning, and execution

Testable engine logic


## Prerequisites

    ```Rust (1.70+)```

# Installation

Clone the repository and build the binary:
```bash

git clone <repository-url>
cd orchestrator
cargo build --release```

The executable will be at target/release/orchestrator.
You can move it to a directory in your $PATH, e.g.:
```bash

cp target/release/orchestrator ~/.local/bin/```

## Usage
### 1. Create a specification file

A spec is a JSON file describing your project. Example myapp.json:
```json

{
  "spec_version": 1,
  "language": "javascript",
  "framework": "react",
  "features": ["tailwind", "eslint"]```

Supported values depend on the data seeded into the registry (see migrations/01_initial.sql).

### 2. Generate an execution plan

```bash

orchestrator plan myapp.json -o plan.json
```

This validates the spec against the registry, resolves dependencies, and writes a complete blueprint to plan.json.

### 3. Apply the plan

You can apply a previously generated plan:
```bash

mkdir my-project
cd my-project
orchestrator apply ../plan.json
```
Or apply directly from a spec (which generates the plan in memory and executes it immediately):

```bash

orchestrator apply myapp.json --from-spec
```
The executor will scaffold the project (using the framework’s base command), install dependencies, and apply any configuration mutations (e.g., adding Tailwind to Vite config).

## Project Structure
text
```
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
└── executor.rs    # Executes plans (scaffolding, installs, etc.)
```
The registry database is stored in your system’s config directory (~/.config/orchestrator/registry.db on Linux/macOS).

### How It Works

Spec – user provides a versioned, declarative description.

Engine – pure function that validates compatibility, resolves dependencies, and produces an explicit ExecutionPlan.

Executor – dumb worker that follows the plan: runs shell commands, installs packages, writes files, and applies modifications.

This separation ensures the system is testable, maintainable, and adaptable to future ecosystems.

## Testing

Run the test suite with:
```bash

cargo test
```
The engine tests use an in‑memory SQLite database (via tempfile) and verify resolution logic.
Extending the Registry

To add new frameworks, features, or dependencies, edit the seed data in migrations/01_initial.sql and rebuild.
For production use, you may want to implement a mechanism to fetch registry updates without rebuilding the binary.


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
└── executor.rs    # Executes plans (scaffolding, installs, etc.)

