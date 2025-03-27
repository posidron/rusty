# Rusty Language

A JavaScript-like scripting language implemented in Rust.

## Overview

Rusty is a simple yet powerful scripting language that features:

- JavaScript-like syntax
- Variables and basic data types
- Functions and control flow
- A built-in standard library
- Interactive REPL

## Getting Started

### Building the Project

```bash
# Clone the repository
git clone https://github.com/yourusername/rusty.git
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

## Language Features

### Variables and Data Types

```
// Variables
var x = 42;
var name = "Rusty";
var is_cool = true;
var nothing = nil;

// Printing values
print x;
print name;
```

### Control Flow

```
// If-else statement
if (x > 10) {
    print "Greater than 10";
} else {
    print "Less than or equal to 10";
}

// While loop
var counter = 0;
while (counter < 5) {
    print counter;
    counter = counter + 1;
}
```

### Functions

```
// Function definition
fun add(a, b) {
    return a + b;
}

// Function call
var result = add(5, 3);
print result;
```

### Operations

```
// Arithmetic
var a = 5 + 3 * 2;
var b = (5 + 3) * 2;

// Comparison
var c = a > b;
var d = a == b;

// Logical
var e = true && false;
var f = true || false;
```

## Standard Library

Rusty comes with a built-in standard library that provides various functions:

- Math functions: `random`, `random_range`, `abs`, `round`, `floor`, `ceil`, `min`, `max`
- String functions: `len`, `upper`, `lower`, `as_string`
- Time functions: `time`

See [Standard Library Documentation](docs/stdlib.md) for details.

## Examples

Check out the examples directory for sample scripts:

- [Hello World](examples/hello_world.rs)
- [Standard Library Demo](examples/stdlib_demo.ry)

## REPL Commands

The interactive REPL supports the following commands:

- `.exit` - Exit the REPL
- `.help` - Show help message
- `.clear` - Clear the screen
- `.load <filename>` - Load and execute a script file

## License

MIT
