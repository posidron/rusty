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

Rusty implements object-oriented programming through namespaces, method calls, and property access.

### Namespaces Overview

Namespaces are collections of related functions and values that are accessed using constructor-like functions.

```rusty
var math = Math();      // Creates a Math namespace instance
var arr = Array();      // Creates an Array namespace instance
var str = String();     // Creates a String namespace instance
```

### Method Calls

Methods within a namespace are called using dot notation followed by parentheses and arguments:

```rusty
var math = Math();
print math.abs(-42);       // Calling the abs method with argument -42

var str = String();
print str.upper("hello");   // Calling the upper method

var arr = Array();
var numbers = arr.create();
numbers = arr.push(numbers, 1);
print arr.length(numbers);    // Calling the length method
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
var math = Math();

// Constants
math.PI    // 3.141592653589793
math.E     // 2.718281828459045

// Methods
math.random()             // Returns a random number between 0 and 1
math.random_range(1, 100) // Returns a random integer between 1 and 100
math.abs(-42)             // Returns 42
math.round(3.7)           // Returns 4
math.floor(3.7)           // Returns 3
math.ceil(3.2)            // Returns 4
math.min(5, 10)           // Returns 5
math.max(5, 10)           // Returns 10
```

### String Namespace

The String namespace provides functions for manipulating strings.

```rusty
var str = String();

// Methods
str.length("hello")        // Returns 5
str.upper("hello")         // Returns "HELLO"
str.lower("HELLO")         // Returns "hello"
str.string(42)             // Returns "42"
```

### Array Namespace

The Array namespace provides functions for working with arrays.

```rusty
var arr = Array();

// Create an array
var numbers = arr.create();  // Empty array - NOTE: Array() directly doesn't create an array
numbers = arr.push(numbers, 1);
numbers = arr.push(numbers, 2);
numbers = arr.push(numbers, 3);

// Methods
arr.create()                // Creates a new empty array
arr.length(numbers)         // Returns 3
arr.push(numbers, 4)        // Returns [1, 2, 3, 4]
arr.pop(numbers)            // Removes and returns the last element
arr.get(numbers, 0)         // Returns 1 (first element)
arr.set(numbers, 1, 99)     // Returns [1, 99, 3]
arr.concat(arr1, arr2)      // Combines two arrays
arr.join(numbers, ", ")     // Joins array elements into a string with separator
```

### File Namespace

The File namespace provides functions for file operations.

```rusty
var file = File();

// Methods
file.read("path/to/file.txt")             // Reads file content as string
file.write("path/to/file.txt", "content") // Writes string to file
file.append("path/to/file.txt", "more")   // Appends string to file
file.exists("path/to/file.txt")           // Checks if file exists
file.delete("path/to/file.txt")           // Deletes the file
```

### Time Namespace

The Time namespace provides time-related functions.

```rusty
var time = Time();

// Methods
time.now()  // Returns current time in milliseconds since epoch
```

### JSON Namespace

The JSON namespace provides functions for working with JSON data.

```rusty
var json = JSON();

// Methods
json.parse('{"name":"John","age":30}')  // Parses JSON string to Rusty value
json.stringify(value)                   // Converts Rusty value to JSON string
```

### Regex Namespace

The Regex namespace provides functions for working with regular expressions.

```rusty
var regex = Regex();

// Methods
regex.new("[0-9]+")                     // Creates a new regex pattern
regex.test(pattern, "123")              // Tests if string matches pattern
regex.match(pattern, "abc123def456")    // Finds all matches of pattern in string
regex.replace(pattern, text, "X")       // Replaces all occurrences of pattern
regex.split(pattern, text)              // Splits string by pattern
regex.capture(pattern, text)            // Gets capture groups from first match
regex.is_valid("[0-9]+")                // Checks if pattern is valid
regex.escape("a.b*c")                   // Escapes special characters in string
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
6. Array creation requires using `Array().create()` rather than a direct `Array()` call

Despite these limitations, Rusty provides a clean, intuitive way to write organized, object-oriented code.
