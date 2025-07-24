# Rust Task Manager CLI

![CI](https://github.com/macrandomer/rust-task-manager/actions/workflows/ci.yml/badge.svg)

A fast, reliable command-line task manager written in Rust.  
Perfect for showcasing on your CV, GitHub portfolio, or as a daily productivity tool.

---

## Features

- **Add, list, mark as done, and delete tasks**
- **Persistent storage** using a local JSON file
- **Clear, contextual error messages** for easy debugging
- **Thorough integration and unit tests**
- **Continuous Integration** (format, lint, build, test)
- **VS Code integration** for seamless development
- **CLI metadata** (version, author, help)
- **Modern Rust practices**: modules, error handling, serde, chrono

---

## Quickstart

### Install from Source
git clone https://github.com/macrandomer/rust-task-manager
cd rust-task-manager
cargo install --path .
text

This installs the `task` command globally on your system.

### Basic Usage 

- To add a new task 
task add "Write blog post"

- To list all tasks
task list

- To mark task #1 as done
task done 1

- To delete task #1
task delete 1

### Example Output

task add "Finish CV"
 Added task 1

task list
 Finish CV (created 2025-07-24T12:00:00Z)

task done 1
 Marked task 1 done

ask list
 Finish CV (created 2025-07-24T12:00:00Z)


## Development

### Clone and Build

git clone https://github.com/macrandomer/rust-task-manager
cd rust-task-manager
cargo build

### Run Tests

cargo test

Tests include both unit and integration tests, with isolated temp directories for CLI testing.

### View API Documentation

cargo doc --open

### VS Code Integration

Open the project in VS Code for:
- **Auto-formatting on save**
- **Rust Analyzer** linting and completion
- **Clean project structure**

---

## CI/CD & Quality

This project uses **GitHub Actions** to enforce:
- **Rustfmt** code style
- **Clippy** lints
- **Build** on push/PR
- **Test** coverage

![CI Badge](https://github.com/macrandomer/rust-task-manager/actions/workflows/ci.yml/badge.svg)

---

## Project Structure

rust-task-manager/
├── .github/
│ └── workflows/
│ └── ci.yml # GitHub Actions CI
├── src/
│ ├── commands.rs # Core task logic
│ ├── lib.rs # Library root
│ └── main.rs # CLI entrypoint
├── tests/
│ └── cli.rs # Integration tests
├── .vscode/
│ └── settings.json # VS Code config
├── Cargo.toml # Package config
└── README.md # This file
text

---

## Contributing

Contributions are welcome! Please open an issue or pull request on GitHub.

---

## License

MIT © Mohammed Abdul Aziz
---

## Author

**Mohammed Abdul Aziz**  
[Email](mailto:mohdabdul532@gmail.com)  
[GitHub](https://github.com/macrandomer)