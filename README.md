# Rusty

A JavaScript-like scripting language implemented in Rust, designed to provide a safe and performant alternative to JavaScript.

## Features

- Lexical analysis (tokenization)
- Abstract Syntax Tree (AST) parsing
- Interpreter for executing code
- Support for:
  - Variables and assignments
  - Functions and function calls
  - Control flow (if/else, while loops)
  - Basic arithmetic operations
  - String operations
  - Boolean operations
  - Comments

## Example

```javascript
// Variable declaration and printing
var greeting = "Hello, World!";
print greeting;

// Function definition and usage
fun add(a, b) {
    return a + b;
}
var result = add(5, 3);
print result;

// Conditional statements
if (result > 5) {
    print "Result is greater than 5";
} else {
    print "Result is less than or equal to 5";
}

// Loops
var counter = 0;
while (counter < 3) {
    print counter;
    counter = counter + 1;
}
```

## Building and Running

1. Make sure you have Rust installed (https://rustup.rs/)
2. Clone this repository
3. Build the project:
   ```bash
   cargo build
   ```
4. Run the example:
   ```bash
   cargo run --example hello_world
   ```

## Project Structure

- `src/lexer.rs`: Tokenizes source code into tokens
- `src/ast_parser.rs`: Parses tokens into an Abstract Syntax Tree
- `src/interpreter.rs`: Executes the AST
- `examples/`: Contains example code demonstrating the language features

## Safety Features

This implementation leverages Rust's type system and ownership model to prevent:
- Memory leaks
- Buffer overflows
- Null pointer dereferences
- Data races
- Other common memory-related bugs

## Future Improvements

- [ ] Add support for classes and objects
- [ ] Implement standard library functions
- [ ] Add support for arrays and objects
- [ ] Implement error handling with try/catch
- [ ] Add support for modules and imports
- [ ] Implement garbage collection
- [ ] Add support for async/await
- [ ] Create a REPL (Read-Eval-Print Loop)
