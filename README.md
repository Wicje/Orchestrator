""src/
├── main.rs
├── cli.rs
├── spec.rs
├── registry/
│   ├── mod.rs
│   ├── models.rs
│   └── sqlite.rs
├── engine.rs
├── plan.rs
└── executor.rs""


#src/main.rs – Entry Point
rust

mod cli;
mod engine;
mod executor;
mod plan;
mod registry;
mod spec;

fn main() {
    if let Err(e) = cli::Cli::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

Step 6 – Build and Test

Run the following commands in your project root:
bash

# Build the project
cargo build --release

# Run tests (the engine tests we wrote)
cargo test

All tests should pass.
Step 7 – Try It Out
Create a spec file myapp.json:
json

{
  "spec_version": 1,
  "language": "javascript",
  "framework": "react",
  "features": ["tailwind", "eslint"]
}

Generate a plan:
bash

./target/release/orchestrator plan myapp.json -o plan.json

This creates plan.json with dependencies etc.
Apply the plan (in a new directory):
bash

mkdir testapp
cd testapp
../target/release/orchestrator apply ../plan.json

Or apply directly from spec:
bash

../target/release/orchestrator apply ../myapp.json --from-spec

The executor will run npm create vite@latest . -- --template react, then install tailwindcss and eslint as dev dependencies.
