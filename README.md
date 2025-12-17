# Minigrep

A simple command-line tool written in Rust that searches for a query string in a specified file. This project is based on the "minigrep" example from the [Rust Programming Language Book](https://doc.rust-lang.org/book/).

## Features

- Case-insensitive text search
- Command-line interface
- Reads from files specified by the user

## Installation

Ensure you have Rust installed on your system. You can install Rust from [rustup.rs](https://rustup.rs/).

Clone the repository and build the project:

```bash
cd minigrep
cargo build --release
```

## Usage

Run the tool with a query and a file path:

```bash
cargo run <query> <file_path>
```

For example:

```bash
cargo run kantha ./geetgovind.txt
```

This will search for the word "kantha" in the file `geetgovind.txt` and print the matching lines.

## How It Works

- The tool accepts two command-line arguments: the query string and the file path.
- It reads the entire file into memory.
- It performs a case-insensitive search for the query in each line of the file.
- Matching lines are printed to the console.

## Dependencies

- `fs` crate for file system operations (included in the standard library)

## Testing

Run the tests with:

```bash
cargo test
```
