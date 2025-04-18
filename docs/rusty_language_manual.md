# Rusty Language Manual

This manual covers all aspects of the Rusty programming language, including syntax, grammar, standard library, and object-oriented features.

## Table of Contents

- [Rusty Language Manual](#rusty-language-manual)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Language Syntax](#language-syntax)
    - [Lexical Structure](#lexical-structure)
      - [Comments](#comments)
      - [String Literals](#string-literals)
      - [Number Literals](#number-literals)
      - [Boolean Literals](#boolean-literals)
      - [Nil Literal](#nil-literal)
    - [Grammar](#grammar)
      - [Program Structure](#program-structure)
      - [Statements](#statements)
      - [Expressions](#expressions)
    - [Data Types](#data-types)
    - [Control Flow](#control-flow)
      - [Conditional Execution](#conditional-execution)
      - [Loops](#loops)
    - [Functions](#functions)
    - [Scope Rules](#scope-rules)
    - [Operators](#operators)
      - [Arithmetic Operators](#arithmetic-operators)
      - [Comparison Operators](#comparison-operators)
      - [Logical Operators](#logical-operators)
      - [Assignment Operator](#assignment-operator)
      - [Property Access Operator](#property-access-operator)
      - [Operator Precedence](#operator-precedence)
  - [Object-Oriented Programming](#object-oriented-programming)
    - [Namespaces Overview](#namespaces-overview)
    - [Method Calls](#method-calls)
    - [Property Access](#property-access)
    - [Creating Custom Namespaces](#creating-custom-namespaces)
  - [Standard Library](#standard-library)
    - [Math Namespace](#math-namespace)
    - [String Namespace](#string-namespace)
    - [Array Namespace](#array-namespace)
    - [File Namespace](#file-namespace)
    - [Time Namespace](#time-namespace)
    - [JSON Namespace](#json-namespace)
    - [Regex Namespace](#regex-namespace)
  - [Best Practices](#best-practices)
  - [Examples](#examples)
    - [Basic Namespace Example](#basic-namespace-example)
    - [String Manipulation Example](#string-manipulation-example)
  - [Implementation Details](#implementation-details)
  - [Limitations](#limitations)
    - [Array Handling](#array-handling)

## Introduction

Rusty is a modern object-oriented scripting language implemented in Rust. It features:

- JavaScript-like syntax
- Dynamic typing
- First-class functions
- Object-oriented programming with namespaces
- Built-in support for arrays, strings, files, JSON, and regular expressions
- A clean, readable syntax

## Language Syntax

### Lexical Structure

Rusty's lexical structure consists of the following token types:

- **Keywords**: `var`, `fun`, `if`, `else`, `while`, `return`, `true`, `false`, `nil`, `print`
- **Identifiers**: Names that start with a letter or underscore, followed by letters, digits, or underscores
- **Literals**: Numbers, strings, booleans, and nil
- **Operators**: `+`, `-`, `*`, `/`, `!`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `=`, `&&`, `||`
- **Punctuation**: `(`, `)`, `{`, `}`, `,`, `;`, `.` (dot for property access)

#### Comments

```rusty
// This is a single-line comment
```

#### String Literals

Strings are enclosed in double quotes:

```rusty
"This is a string"
```

String literals support escape sequences like `\"`, `\\`, `\n`, `\r`, `\t`.

#### Number Literals

Numbers can be integers or floating-point:

```rusty
123     // Integer
123.45  // Floating-point
```

#### Boolean Literals

```rusty
true
false
```

#### Nil Literal

```rusty
nil     // Represents no value or absence of value
```

### Grammar

#### Program Structure

A Rusty program consists of a sequence of statements:

```
program     → statement* EOF ;
```

#### Statements

```
statement   → exprStmt
            | printStmt
            | blockStmt
            | varStmt
            | ifStmt
            | whileStmt
            | funStmt
            | returnStmt ;

exprStmt    → expression ";" ;
printStmt   → "print" expression ";" ;
blockStmt   → "{" statement* "}" ;
varStmt     → "var" IDENTIFIER ( "=" expression )? ";" ;
ifStmt      → "if" "(" expression ")" statement ( "else" statement )? ;
whileStmt   → "while" "(" expression ")" statement ;
funStmt     → "fun" IDENTIFIER "(" parameters? ")" blockStmt ;
returnStmt  → "return" expression? ";" ;
parameters  → IDENTIFIER ( "," IDENTIFIER )* ;
```

#### Expressions

```
expression  → assignment ;
assignment  → IDENTIFIER "=" assignment
            | logic_or ;
logic_or    → logic_and ( "||" logic_and )* ;
logic_and   → equality ( "&&" equality )* ;
equality    → comparison ( ( "==" | "!=" ) comparison )* ;
comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term        → factor ( ( "+" | "-" ) factor )* ;
factor      → unary ( ( "*" | "/" ) unary )* ;
unary       → ( "!" | "-" ) unary
            | call ;
call        → property ( "(" arguments? ")" )* ;
property    → primary ( "." IDENTIFIER ( "(" arguments? ")" )? )* ;
arguments   → expression ( "," expression )* ;
primary     → NUMBER | STRING | "true" | "false" | "nil"
            | "(" expression ")"
            | IDENTIFIER
            | array ;
array       → "array" "(" arguments? ")" ;
```

The `property` rule handles both property access and method calls:
- `object.property` - Simple property access
- `object.method(args)` - Method call

### Data Types

Rusty is dynamically typed. Variables can hold any of these values:

- **Number**: Double-precision floating-point numbers
- **String**: Text strings
- **Boolean**: `true` or `false`
- **Nil**: Represents absence of a value
- **Function**: Function values (first-class functions)
- **Array**: Ordered collection of values, created with `Array().create()`
- **Object**: Collections of key-value pairs
- **Namespace**: Named collections of related functions and constants

### Control Flow

#### Conditional Execution

```rusty
if (condition) {
    // code executed if condition is true
} else {
    // code executed if condition is false
}
```

#### Loops

```rusty
while (condition) {
    // code executed repeatedly while condition is true
}
```

### Functions

Functions are defined with the `fun` keyword:

```rusty
fun add(a, b) {
    return a + b;
}
```

Functions are called with arguments in parentheses:

```rusty
var result = add(5, 3);
```

Functions are first-class values and can be assigned to variables or passed as arguments.

### Scope Rules

Rusty uses lexical scoping with block scope. Variables declared inside a block are only accessible within that block and its nested blocks.

```rusty
{
    var x = 10;
    {
        var y = 20;
        print x;      // 10
        print y;      // 20
    }
    print x;          // 10
    print y;          // Error: y is not defined in this scope
}
```

### Operators

#### Arithmetic Operators
- `+`: Addition (numbers) or concatenation (strings)
- `-`: Subtraction
- `*`: Multiplication
- `/`: Division

#### Comparison Operators
- `==`: Equal to
- `!=`: Not equal to
- `>`: Greater than
- `>=`: Greater than or equal to
- `<`: Less than
- `<=`: Less than or equal to

#### Logical Operators
- `&&`: Logical AND
- `||`: Logical OR
- `!`: Logical NOT

#### Assignment Operator
- `=`: Assigns a value to a variable

#### Property Access Operator
- `.`: Accesses a property or method of an object

#### Operator Precedence

From highest to lowest:
1. Grouping: `()`
2. Property access: `.`
3. Function calls: `func()`
4. Unary: `-x`, `!x`
5. Multiplication/Division: `*`, `/`
6. Addition/Subtraction: `+`, `-`
7. Comparison: `<`, `<=`, `>`, `>=`
8. Equality: `==`, `!=`
9. Logical AND: `&&`
10. Logical OR: `||`
11. Assignment: `=`

## Object-Oriented Programming

Rusty implements object-oriented programming through namespaces, method calls, and property access following the JavaScript pattern.

### Namespaces Overview

Namespaces in Rusty provide collections of related functions and constants. Namespaces are accessed directly like in JavaScript:

```rusty
// Accessing namespace methods directly
Math.abs(-42);             // Call Math.abs method with -42 as argument
String.upper("hello");     // Call String.upper method
Array.length([1, 2, 3]);   // Call Array.length method

// Accessing namespace constants
var pi = Math.PI;          // Access Math.PI constant

// Creating arrays with constructor or literals
var arr1 = Array(1, 2, 3); // Using constructor
var arr2 = [1, 2, 3];      // Using literal syntax

// String conversion
var str = String(42);      // Convert number to string
```

### Method Calls

Methods within a namespace are called using dot notation followed by parentheses and arguments:

```rusty
// Static method calls
Math.abs(-42);              // Call abs method with argument -42
String.upper("hello");      // Call upper method
Array.length([1, 2, 3]);    // Call length method on an array

// Method chaining example
var data = JSON.parse(File.read("data.json"));
var formatted = String.upper(Array.get(data, 0));
```

### Property Access

Properties (constants, values) within a namespace are accessed using dot notation:

```rusty
var math = Math();
print math.PI;             // Accessing the PI constant
```

### Creating Custom Namespaces

You can create your own namespaces to organize your code:

```rusty
fun create_utils_namespace() {
    var utils = Array().create();

    // Define functions within the namespace
    fun is_even(n) {
        return n % 2 == 0;
    }

    fun is_odd(n) {
        return n % 2 == 1;
    }

    fun factorial(n) {
        if (n <= 1) {
            return 1;
        }
        return n * factorial(n - 1);
    }

    // Add methods to the namespace
    utils = Array().push(utils, Array().create());
    utils = Array().set(Array().get(utils, 0), 0, "is_even");
    utils = Array().set(Array().get(utils, 0), 1, is_even);

    var second_method = Array().create();
    second_method = Array().push(second_method, "is_odd");
    second_method = Array().push(second_method, is_odd);
    utils = Array().push(utils, second_method);

    var third_method = Array().create();
    third_method = Array().push(third_method, "factorial");
    third_method = Array().push(third_method, factorial);
    utils = Array().push(utils, third_method);

    return utils;
}

// Create and use the namespace
var Utils = create_utils_namespace();
print Utils.is_even(4);    // true
print Utils.is_odd(4);     // false
print Utils.factorial(5);  // 120
```

## Standard Library

Rusty comes with a built-in standard library that provides several namespaces:

### Math Namespace

The Math namespace provides mathematical functions and constants.

```rusty
// Constants accessible as static properties
Math.PI    // 3.141592653589793
Math.E     // 2.718281828459045

// Static methods
Math.random()             // Returns a random number between 0 and 1
Math.random_range(1, 100) // Returns a random integer between 1 and 100
Math.abs(-42)             // Returns 42
Math.round(3.7)           // Returns 4
Math.floor(3.7)           // Returns 3
Math.ceil(3.2)            // Returns 4
Math.min(5, 10)           // Returns 5
Math.max(5, 10)           // Returns 10

// Using Math methods in expressions
var area = Math.PI * Math.pow(radius, 2);
var rounded = Math.round(3.7);
```

### String Namespace

The String namespace provides functions for manipulating strings.

```rusty
// Creating/converting strings with constructor
var numAsStr = String(42);           // Converts number to string: "42"
var boolAsStr = String(true);        // Converts boolean to string: "true"

// Using static String methods
String.length("hello")        // Returns 5
String.upper("hello")         // Returns "HELLO"
String.lower("HELLO")         // Returns "hello"
String.string(42)             // Alternative way to convert to string: "42"

// Combining with other operations
var rounded = Math.round(3.7);
print "Rounded value: " + String(rounded);  // "Rounded value: 4"
```

### Array Namespace

The Array namespace provides functions for working with arrays.

```rusty
// Creating arrays
var empty = Array();            // Creates an empty array
var numbers = Array(1, 2, 3);   // Creates array with values [1, 2, 3]
var mixed = Array("hello", 42, true); // Mixed types

// Get the array methods namespace
var am = ArrayMethods();

// Array operations
am.length(numbers)         // Returns 3
am.push(numbers, 4)        // Returns [1, 2, 3, 4]
am.pop(numbers)            // Removes and returns the last element
am.get(numbers, 0)         // Returns 1 (first element)
am.set(numbers, 1, 99)     // Returns [1, 99, 3]
am.concat(arr1, arr2)      // Combines two arrays
am.join(numbers, ", ")     // Joins array elements into a string
```

### File Namespace

The File namespace provides functions for file operations.

```rusty
// Static methods for file operations
File.read("path/to/file.txt")             // Reads file content as string
File.write("path/to/file.txt", "content") // Writes string to file
File.append("path/to/file.txt", "more")   // Appends string to file
File.exists("path/to/file.txt")           // Checks if file exists
File.delete("path/to/file.txt")           // Deletes the file

// Example usage
if (File.exists("data.txt")) {
    var content = File.read("data.txt");
    print "File content: " + content;
} else {
    File.write("data.txt", "Hello, world!");
}
```

### Time Namespace

The Time namespace provides time-related functions.

```rusty
// Get current timestamp
var timestamp = Time.now();  // Returns current time in milliseconds since epoch

// Example of calculating elapsed time
var start = Time.now();
// ... do some work ...
var end = Time.now();
var elapsed = end - start;
print "Operation took " + String(elapsed) + " ms";
```

### JSON Namespace

The JSON namespace provides functions for working with JSON data.

```rusty
// Parse JSON string to Rusty value
var data = JSON.parse('{"name":"John","age":30}');

// Access parsed data
var name = Array.get(Array.get(data, 0), 1);  // "John"
var age = Array.get(Array.get(data, 1), 1);   // 30

// Convert Rusty value to JSON string
var person = [["name", "Alice"], ["age", 25]];
var jsonStr = JSON.stringify(person);  // '{"name":"Alice","age":25}'
```

### Regex Namespace

The Regex namespace provides functions for working with regular expressions.

```rusty
// Create and use regex patterns
var pattern = Regex.new("[0-9]+");
var isMatch = Regex.test(pattern, "123");       // true
var noMatch = Regex.test(pattern, "abc");       // false

// Find all matches
var matches = Regex.match(pattern, "abc123def456");  // ["123", "456"]

// Replace matches
var replaced = Regex.replace(pattern, "Order #123 contains 4 items", "X");
// "Order #X contains X items"

// Other operations
var parts = Regex.split(pattern, "abc123def456");      // ["abc", "def", ""]
var captured = Regex.capture(pattern, "abc123def");    // ["123"]
var isValid = Regex.is_valid("[0-9]+");               // true
var escaped = Regex.escape("a.b*c");                  // "a\.b\*c"
```

## Best Practices

When using Rusty language, consider these best practices:

1. **Consistent Naming**: Use snake_case for method names to maintain consistency with the standard library
2. **Namespacing**: Group related functions into logical namespaces
3. **Immutability**: Remember that object operations return new copies rather than modifying in place
4. **Documentation**: Document the purpose and usage of your namespaces and methods
5. **Error Handling**: Check for error cases and handle them gracefully
6. **Code Organization**: Break complex tasks into smaller, focused functions

## Examples

See the `examples/` directory for complete demonstrations of Rusty language features:

- `examples/full_oop_demo.ry` - Comprehensive demo of all namespaces
- `examples/math_namespace.ry` - Focused example of Math namespace
- `examples/string_namespace.ry` - Focused example of String namespace
- `examples/array_namespace.ry` - Focused example of Array namespace
- `examples/file_namespace.ry` - Focused example of File namespace
- `examples/time_namespace.ry` - Focused example of Time namespace
- `examples/json_namespace.ry` - Focused example of JSON namespace
- `examples/regex_namespace.ry` - Focused example of Regex namespace
- `examples/custom_namespace.ry` - Example of creating custom namespaces
- `examples/shapes_namespace_demo.ry` - Complex 2D shapes library implementation

### Basic Namespace Example

```rusty
// Using Math namespace
var math = Math();

// Calculate factorial using recursion
fun factorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

var n = 5;
var str = String();
print "Factorial of " + str.string(n) + " is " + str.string(factorial(n));

// Use arrays with namespace methods
var arr = Array();
var numbers = arr.create();
numbers = arr.push(numbers, 1);
numbers = arr.push(numbers, 2);
numbers = arr.push(numbers, 3);
print "Original array: " + str.string(numbers);

// Get array length
var len = arr.length(numbers);
print "Array length: " + str.string(len);

// Add element
var extended = arr.push(numbers, 4);
print "After push: " + str.string(extended);
```

### String Manipulation Example

```rusty
// Using String namespace
var str = String();
var text = "hello, world";

// Convert to uppercase
var upper = str.upper(text);
print upper;  // "HELLO, WORLD"

// Convert to lowercase
var lower = str.lower(upper);
print lower;  // "hello, world"

// Get string length
var len = str.length(text);
print "Length: " + str.string(len);  // "Length: 12"

// Use regex to find matches
var regex = Regex();
var pattern = regex.new("[a-z]+");
var matches = regex.match(pattern, text);
print "Words: " + str.string(matches);  // ["hello", "world"]
```

## Implementation Details

Namespaces in Rusty are implemented using:

1. The `Value::Namespace` variant in the interpreter's `Value` enum
2. Property access via the `Expr::Get` expression type
3. Method calls via the `Expr::Method` expression type

Under the hood, namespaces are stored as hash maps of string keys to values, allowing for efficient lookup of properties and methods.

## Limitations

Current limitations of the Rusty language:

1. No direct inheritance or class-based OOP
2. No private/protected access modifiers for encapsulation
3. Method calls are not optimized for chaining (each call returns a new object)
4. No exception handling mechanism
5. Limited standard library compared to mature languages

Despite these limitations, Rusty provides a clean, intuitive way to write organized, object-oriented code.

### Array Handling

Rusty provides JavaScript-style array creation and manipulation:

```rusty
// Creating arrays with literals (recommended)
var empty = [];                  // Empty array
var numbers = [1, 2, 3, 4, 5];   // Array of numbers
var mixed = ["hello", 42, true]; // Mixed types
var nested = [[1, 2], [3, 4]];   // Nested arrays

// Alternatively, create arrays with the Array constructor
var empty2 = Array();            // Empty array
var numbers2 = Array(1, 2, 3);   // Array with values [1, 2, 3]

// Using static methods on the Array namespace (exactly like JavaScript)
Array.length(numbers);           // Returns 5
Array.push(numbers, 6);          // Returns [1, 2, 3, 4, 5, 6]
Array.pop(numbers);              // Removes and returns the last element
Array.get(numbers, 0);           // Returns 1 (first element)
Array.set(numbers, 1, 99);       // Returns [1, 99, 3, 4, 5]
Array.concat([1, 2], [3, 4]);    // Returns [1, 2, 3, 4]
Array.join(numbers, ", ");       // Joins array elements into a string

// Direct use with literals
Array.length([1, 2, 3]);         // Returns 3
```
