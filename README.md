# Rusty Language

A JavaScript-like scripting language implemented in Rust.

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

### File I/O

```
// Unified file function with different modes:

// Check if a file exists
var exists = file("data.txt", "e");

// Write to a file
file("data.txt", "w", "Hello, World!");

// Read from a file
var content = file("data.txt", "r");
print content;

// Append to a file
file("data.txt", "a", "\nMore data here");

// Delete a file
file("data.txt", "d");
```

### Arrays

```
// Creating arrays
var numbers = array(1, 2, 3, 4, 5);

// Array operations
var len = length(numbers);
var first = get(numbers, 0);
var new_array = set(numbers, 1, 99);
var extended = push(numbers, 6);
var popped = pop(extended);
var joined = join(numbers, ", ");
var combined = concat(numbers, array(6, 7, 8));
```

### JSON

```
// Parsing JSON strings
var data = json_parse("{\"name\": \"John\", \"age\": 30}");

// Accessing JSON data (as arrays of key-value pairs)
var first_pair = get(data, 0);
var key = get(first_pair, 0);
var value = get(first_pair, 1);

// Converting values to JSON
var obj = array();
obj = push(obj, array("name", "Alice"));
obj = push(obj, array("age", 25));
var json = json_stringify(obj);
```

Rusty also includes a set of helper functions for more convenient JSON manipulation:

```
// Create a new object
var person = object_new();
person = object_set(person, "name", "John");
person = object_set(person, "age", 30);

// Access properties
var name = object_get(person, "name");

// Save/load JSON
save_json(person, "person.json");
var loaded = load_json("person.json");
```

### Regular Expressions

```
// Creating regex patterns
var regex = regex_new("\\d+");

// Testing if a string matches a pattern
var is_match = regex_test(regex, "123");

// Finding all matches
var matches = regex_match_all(regex, "abc123def456");

// Replacing text
var result = regex_replace_all(regex_new("\\s+"), "hello  world", "_");

// Splitting strings
var parts = regex_split(regex_new(",\\s*"), "apple, orange, banana");

// Capturing groups
var captures = regex_capture(regex_new("(\\w+):(\\d+)"), "port:8080");
```

Rusty includes helper functions for easier regex usage:

```
// Create and use regex with caching
var is_number = match("^\\d+$", "123");

// Count matches
var vowel_count = match_count("[aeiou]", "hello world");

// Replace first occurrence only
var replaced = replace_first("the", "the quick the fox", "a");

// Work with named captures
var date_parts = extract("(\\d{4})-(\\d{2})-(\\d{2})", "2023-04-15",
                         array("year", "month", "day"));
```

See [Standard Library Documentation](docs/stdlib.md) for more details.

## Standard Library

Rusty comes with a built-in standard library that provides various functions:

- Math functions: `random`, `random_range`, `abs`, `round`, `floor`, `ceil`, `min`, `max`
- String functions: `len`, `upper`, `lower`, `as_string`
- Time functions: `time`
- File I/O functions: Unified `file(path, mode, [content])` function with modes: "r" (read), "w" (write), "a" (append), "e" (exists), "d" (delete)
- Array functions: `array`, `length`, `push`, `pop`, `get`, `set`, `concat`, `join`
- JSON functions: `json_parse`, `json_stringify`
- Regex functions: `regex_new`, `regex_test`, `regex_match_all`, `regex_replace_all`, `regex_split`, `regex_capture`, `regex_is_valid`, `regex_escape`

See [Standard Library Documentation](docs/stdlib.md) for details.

Additionally, Rusty includes sets of helper functions:

- JSON utilities: `object_new`, `object_set`, `object_get`, `object_has`, `object_keys`, `to_json`, `from_json`, `save_json`, `load_json`
- Regex utilities: `regex`, `match`, `match_all`, `match_count`, `is_valid_pattern`, `replace_all`, `replace_first`, `split`, `capture`, `extract`, common pattern constants, validators (`is_email`, `is_url`, etc.)

See [Standard Library Documentation](docs/stdlib.md) for more details on these utilities.

## Examples

Check out the examples directory for sample scripts:

- [Hello World](examples/hello_world.rs): Basic "Hello World" example
- [Standard Library Demo](examples/stdlib_demo.ry): Demonstrates standard library functions
- [File I/O Demo](examples/file_unified_demo.ry): Shows file operations
- [Array Demo](examples/array_demo.ry): Demonstrates array manipulation
- [JSON Basic Demo](examples/json_basic_demo.ry): Simple JSON parsing and serialization
- [JSON Utilities](examples/json_utils.ry): Helper functions for easier JSON handling
- [JSON with Files](examples/json_file_example.ry): Configuration management with JSON files
- [Regex Demo](examples/regex_demo.ry): Basic regular expression operations
- [Log Analyzer](examples/log_analyzer.ry): Practical example of regex for log analysis
