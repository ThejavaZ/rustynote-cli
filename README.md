# rustynote-cli

> A lightweight, fast task and note manager CLI for Linux, built with vanilla Rust.

`rustynote-cli` is a command-line tool designed to help you manage your daily tasks, notes, and reminders directly from your terminal with lightning-fast execution and absolute memory safety.

Built with **core Rust idioms, zero unnecessary overhead, and pure standard library tooling** in mind.

## Features

- Fast task and note creation (`add`)
- Clear, formatted listing of pending and completed tasks (`list`)
- Task completion toggle (`done`)
- Secure item removal (`delete`)
- Local JSON file persistence (`notes.json`) utilizing standard I/O streams
- Pattern-matched command handling with safe error propagation (`Result` and `Option`)
- Zero heavy web framework dependencies — powered entirely by Rust's `std` library
- Compile-time memory safety guaranteed by Rust's ownership and borrowing model

## Quick Start

### Installation

Clone the repository and build the binary using Cargo (Rust's package manager):

```bash
git clone https://github.com/ThejavaZ/rustynote-cli.git
cd rustynote-cli
cargo build --release

```

### Run

You can run the application directly via Cargo:

```bash
cargo run -- add "Learn Rust ownership rules"
cargo run -- list

```

Or install it locally to your cargo path:

```bash
cargo install --path .
rustynote-cli list

```

## CLI Commands

Available CLI actions and arguments:

| Command  | Arguments | Description                        |
| -------- | --------- | ---------------------------------- |
| `add`    | `<text>`  | Create a new task or note          |
| `list`   | None      | Display all stored tasks and notes |
| `done`   | `<id>`    | Mark a specific task as completed  |
| `delete` | `<id>`    | Remove a task or note by its ID    |
| `help`   | None      | Show usage instructions and help   |

## Project Structure

```text
rustynote-cli/
├── Cargo.toml          # Package configuration and dependencies
├── src/
│   ├── main.rs         # CLI entry point and argument dispatcher
│   ├── models.rs       # Structs and Enums (Task, Status)
│   ├── storage.rs      # File persistence (JSON read/write via std::fs)
│   └── commands.rs     # Command business logic handlers
└── notes.json          # Local persistent database (created automatically)

```

## Roadmap

### Phase 1 — Foundation & CLI Arguments ✅

- [ ] Project initialization with `cargo`
- [ ] Basic argument capture via `std::env::args`
- [ ] Core command router with pattern matching (`match`)

### Phase 2 — Data Structures & Core Logic ✅

- [ ] Define `Task` and `Note` structs with unique IDs
- [ ] Implement add, list, and delete logic in memory
- [ ] Implement task completion toggling (`done`)

### Phase 3 — File Persistence ✅

- [ ] Serialize and deserialize data structures to local JSON
- [ ] Handle missing files gracefully with Rust's `Result` and `Option`
- [ ] Ensure atomic file writes to prevent data corruption

### Phase 4 — Robustness & Polish

- [ ] Add input validation and sanitation
- [ ] Implement colorized terminal output using lightweight ANSI escape sequences
- [ ] Comprehensive unit and integration testing (`cargo test`)

## Safety & Memory Management

Unlike managed languages, `rustynote-cli` benefits directly from Rust's core design pillars:

- **No Garbage Collector:** Memory is managed deterministically at compile time through strict ownership rules.
- **No Null Pointer Exceptions:** Absence of null values is enforced via `Option<T>`.
- **Safe Error Handling:** Failures are explicitly handled through `Result<T, E>`, ensuring your CLI never crashes unexpectedly due to unhandled exceptions.

## License

See [`LICENSE`](LICENSE) for details.
