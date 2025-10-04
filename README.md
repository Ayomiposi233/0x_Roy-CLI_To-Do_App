# Rust CLI To-Do App
A simple command-line interface (CLI) application built in Rust for managing a to-do list. This app allows users to create, edit, complete, and delete tasks, with persistence using JSON file storage.

## Features
- __List To-dos__: Display all current tasks with their IDs, titles, and completion status.
- __Add To-do__: Create a new task with an auto-assigned ID.
- __Edit To-do__: Update the title of an existing task by ID.
- __Complete To-do__: Mark a task as completed by ID.
- __Delete To-do__: Remove a task by ID and re-number remaining IDs for consistency.
- __Persistence__: Tasks are saved to and loaded from a todos.json file in the project root.

## Prerequisites
- Rust programming language installed (version 1.81.0 or later recommended).
    - Install via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Installation
1. Clone or download the project repository.
2. Navigate to the project directory:
```
cd todo_cli
```
3. Install dependencies using Cargo:
```
cargo add serde --features derive
cargo add serde_json
```
## Building and Running
1. Build the project:
```
cargo build
```
2. Run the application:
```
cargo run
```
  - The app will display a menu with options (1-6).
  - Interact via keyboard input.
  - Changes are saved automatically after each operation and on exit.

## Usage
Upon running, you'll see the following menu:
```
Todo CLI App
1. List todos
2. Add todo
3. Edit todo
4. Complete todo
5. Delete todo
6. Exit
```
- Select an option by entering the number.
- For add/edit/complete/delete, follow the prompts for title or ID.
- Example:
  - To add: Choose 2, enter a title like "Buy groceries".
  - To complete: Choose 4, enter the ID of the task.
Tasks are stored in `todos.json` for persistence between sessions.
## Dependencies
- __serde__: For serialization/deserialization (with `derive` feature).
- __serde_json__: For JSON handling.

These are specified in `Cargo.toml`:
```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```
## Code Structure
- `src/main.rs`: Contains the main logic, including the `Todo` struct, functions for operations, and the CLI loop.
- `todos.json`: Auto-created file for storing tasks.

## Contributing
This is a beginner-friendly project. Feel free to fork and submit pull requests for improvements.

