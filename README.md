# Orchestrator

A deterministic project bootstrapping engine.

You declare what you want (language, framework, features) — Orchestrator figures out how to build it.

Orchestrator is a Rust-based CLI tool that reads a project specification file and automatically generates and executes a setup plan. Instead of manually running scaffold commands and installing dependencies, you describe your project in a JSON spec file. Orchestrator builds a reproducible plan and applies it for you.

---

## Example

```bash
# Generate a React + Tailwind + ESLint project
orchestrator plan myapp.json -o plan.json
orchestrator apply plan.json
```

---

## Features

- Generate project setup plans from a JSON specification
- Apply plans to scaffold new projects
- Install dependencies and development tools automatically
- Clean separation between specification, planning, and execution
- Testable engine logic

---

## Prerequisites

- Rust 1.70+

---

## Installation

Clone the repository and build the binary:

```bash
git clone <repository-url>
cd orchestrator
cargo build --release
```

The executable will be located at:

```
target/release/orchestrator
```

(Optional) Move it to your PATH:

```bash
cp target/release/orchestrator ~/.local/bin/
```

---

## Usage

### 1. Create a Specification File

Example `myapp.json`:

```json
{
  "spec_version": 1,
  "language": "javascript",
  "framework": "react",
  "features": ["tailwind", "eslint"]
}
```

Supported values depend on the data seeded into the registry (see `migrations/01_initial.sql`).

---

### 2. Generate an Execution Plan

```bash
orchestrator plan myapp.json -o plan.json
```

This validates the spec against the registry, resolves dependencies, and writes a complete blueprint to `plan.json`.

---

### 3. Apply the Plan

Apply a previously generated plan:

```bash
mkdir my-project
cd my-project
orchestrator apply ../plan.json
```

Or apply directly from a spec:

```bash
orchestrator apply myapp.json --from-spec
```

The executor will scaffold the project, install dependencies, and apply required configuration changes.

---

## Project Structure

```
src/
├── main.rs
├── cli.rs
├── spec.rs
├── registry/
│   ├── mod.rs
│   ├── models.rs
│   └── sqlite.rs
├── engine.rs
├── plan.rs
└── executor.rs
```

The registry database is stored in your system’s config directory:

- Linux/macOS: `~/.config/orchestrator/registry.db`

---

## How It Works

**Spec**  
User provides a versioned, declarative project description.

**Engine**  
Validates compatibility, resolves dependencies, and produces an `ExecutionPlan`.

**Executor**  
Runs shell commands, installs packages, writes files, and applies modifications based on the plan.

---

## Testing

Run the test suite:

```bash
cargo test
```

The engine tests use an in-memory SQLite database and verify dependency resolution logic.

---

## Extending the Registry

To add new frameworks, features, or dependencies:

1. Edit the seed data in `migrations/01_initial.sql`
2. Rebuild the binary

For production use, consider implementing a remote registry update mechanism.
