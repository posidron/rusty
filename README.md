# Rusty Language

A modern object-oriented scripting language implemented in Rust.

## Overview

Rusty is a simple yet powerful scripting language that features:

- JavaScript-like syntax
- Variables and basic data types
- Functions and control flow
- File I/O capabilities
- Array and JSON support
- Regular expression support
- A built-in standard library
- Interactive REPL
- Fully object-oriented approach

## Getting Started

### Building the Project

```bash
# Clone the repository
git clone https://github.com/posidron/rusty.git
cd rusty

# Build the project
cargo build --release
```

### Running the REPL

```bash
# Run in interactive mode
cargo run --bin repl

# Execute a script file
cargo run --bin repl path/to/your/script.ry
```

### Building a Release Executable

To build a standalone executable that you can use directly:

```bash
# Build in release mode
cargo build --release

# The executable will be in the target/release directory
# For Unix/Linux/macOS
./target/release/repl

# For Windows
.\target\release\repl.exe
```

You can copy this executable to a location in your PATH for easy access:

```bash
# For Unix/Linux/macOS
cp ./target/release/repl /usr/local/bin/rusty

# For Windows (using PowerShell):
# Copy-Item .\target\release\repl.exe -Destination "C:\Windows\System32\rusty.exe"
```

Then you can run scripts with:

```bash
# Run the REPL in interactive mode
rusty

# Execute a script file
rusty path/to/your/script.ry
```

## Language Features Overview

Rusty is a fully object-oriented language with a clean, intuitive syntax. It provides the following key features:

- **Variables and Data Types**: Support for numbers, strings, booleans, and nil
- **Object-Oriented Programming**: Namespaces and method calls for clean organization
- **Control Flow**: If-else statements and while loops
- **Functions**: Define and call functions with parameters and return values
- **Operators**: Arithmetic, comparison, and logical operators
- **Standard Library**: Comprehensive built-in functionality

The language uses a consistent object-oriented model where functionality is organized into namespaces like `Math()`, `String()`, `Array()`, etc. Each namespace provides related methods and properties using snake_case naming convention:

```
// Example of OOP style in Rusty
var math = Math();
var result = math.abs(-42);  // => 42

var arr = Array();
var numbers = arr.create();  // Create an array
numbers = arr.push(numbers, 1);  // Arrays are immutable, so methods return new arrays
```

## Standard Library

Rusty comes with a comprehensive standard library organized into namespaces:

- **Math**: Mathematical operations (`abs()`, `random_range()`, etc.) and constants (`PI`, `E`)
- **String**: String manipulation methods (`upper()`, `lower()`, `string()`, etc.)
- **Array**: Array operations (`create()`, `push()`, `length()`, etc.)
- **File**: File I/O operations (`read()`, `write()`, etc.)
- **Time**: Time-related functions (`now()`)
- **JSON**: JSON parsing and serialization (`parse()`, `stringify()`)
- **Regex**: Regular expression support (`new()`, `test()`, `match_all()`, `replace_all()`, `is_valid()`, etc.)

Note: The language follows consistent snake_case naming for all methods (e.g., `random_range()` not `randomRange()`), and array creation requires using the `create()` method (e.g., `Array().create()`).

## Documentation

- [Rusty Language Manual](docs/rusty_language_manual.md) - Comprehensive documentation of all language features
- [Examples](examples/) - Sample scripts demonstrating various language features:
  - [Full OOP Demo](examples/full_oop_demo.ry) - Demonstrates all namespaces
  - [File Namespace Demo](examples/file_namespace_demo.ry) - Shows file operations

## Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test --package rustjs --lib stdlib::string
```
