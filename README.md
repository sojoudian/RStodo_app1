# RStodo_app1

A simple command-line todo list manager written in Rust. This application demonstrates basic Rust concepts including file I/O, error handling, and data serialization.

## Features

- Add new tasks
- Remove tasks by ID
- List all tasks
- Persistent storage using JSON
- Simple command-line interface
- Automatic task ID management

## Prerequisites

Before running this application, make sure you have:

- Rust installed (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/sojoudian/RStodo_app1.git
cd RStodo_app1
```

2. Build the application:
```bash
cargo build --release
```

The executable will be created as `target/release/RStodo_app1`

## Usage

Run the application:
```bash
cargo run
```

You'll see a menu with the following options:
1. Add task
2. Remove task
3. List tasks
4. Exit

### Adding a Task
- Select option 1
- Enter your task description
- The task will be saved with a unique ID

### Removing a Task
- Select option 2
- Enter the ID of the task you want to remove

### Listing Tasks
- Select option 3 to see all current tasks

### Data Persistence
Tasks are automatically saved to `todo_list.json` in the project directory. This means your tasks will persist between program runs.

## Project Structure

```
RStodo_app1/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs         # Application source code
└── todo_list.json      # Task storage file (created on first run)
```

## Dependencies

- serde: For data serialization
- serde_json: For JSON encoding/decoding

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Built with Rust
- Uses serde for JSON serialization
- Inspired by command-line todo applications

## Author

- Maziar

## Version History

- 0.1.0
    - Initial Release
    - Basic CRUD operations
    - File persistence
