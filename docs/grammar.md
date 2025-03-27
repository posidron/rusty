# Rusty Language Grammar

This document describes the formal grammar and language features of the Rusty programming language.

## Lexical Structure

### Tokens

Rusty's lexical structure consists of the following token types:

- **Keywords**: `var`, `fun`, `if`, `else`, `while`, `return`, `true`, `false`, `nil`, `print`
- **Identifiers**: Names that start with a letter or underscore, followed by letters, digits, or underscores
- **Literals**: Numbers, strings, booleans, and nil
- **Operators**: `+`, `-`, `*`, `/`, `!`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `=`, `&&`, `||`
- **Punctuation**: `(`, `)`, `{`, `}`, `,`, `;`

### Comments

```
// This is a single-line comment
```

### String Literals

Strings are enclosed in double quotes:

```
"This is a string"
```

String literals support escape sequences like `\"`, `\\`, `\n`, `\r`, `\t`.

### Number Literals

Numbers can be integers or floating-point:

```
123     // Integer
123.45  // Floating-point
```

### Boolean Literals

```
true
false
```

### Nil Literal

```
nil     // Represents no value or absence of value
```

## Syntax

### Program Structure

A Rusty program consists of a sequence of statements:

```
program     → statement* EOF ;
```

### Statements

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

### Expressions

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
call        → primary ( "(" arguments? ")" )* ;
arguments   → expression ( "," expression )* ;
primary     → NUMBER | STRING | "true" | "false" | "nil"
            | "(" expression ")"
            | IDENTIFIER ;
```

## Semantics

### Variable Declaration and Assignment

Variables must be declared before use with the `var` keyword. They can be declared with or without an initial value.

```
var name;        // Declaration without initialization (value is nil)
var age = 30;    // Declaration with initialization
name = "John";   // Assignment to existing variable
```

### Data Types

Rusty is dynamically typed. Variables can hold any of these values:

- **Number**: Double-precision floating-point numbers
- **String**: Text strings
- **Boolean**: `true` or `false`
- **Nil**: Represents absence of a value
- **Function**: Function values (first-class functions)
- **Array**: Ordered collection of values, created with the `array()` function

### Control Flow

#### Conditional Execution

```
if (condition) {
    // code executed if condition is true
} else {
    // code executed if condition is false
}
```

#### Loops

```
while (condition) {
    // code executed repeatedly while condition is true
}
```

### Functions

Functions are defined with the `fun` keyword:

```
fun add(a, b) {
    return a + b;
}
```

Functions are called with arguments in parentheses:

```
var result = add(5, 3);
```

Functions are first-class values and can be assigned to variables or passed as arguments.

### Scope

Rusty uses lexical scoping with block scope. Variables declared inside a block are only accessible within that block and its nested blocks.

```
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

## Standard Library

Rusty includes a standard library with the following built-in functions:

### Math Functions
- `random()`: Returns a random number between 0 and 1
- `random_range(min, max)`: Returns a random number between min and max
- `abs(x)`: Returns the absolute value of x
- `round(x)`: Rounds x to the nearest integer
- `floor(x)`: Rounds x down to the nearest integer
- `ceil(x)`: Rounds x up to the nearest integer
- `min(a, b)`: Returns the smaller of a and b
- `max(a, b)`: Returns the larger of a and b

### String Functions
- `len(s)`: Returns the length of string s
- `upper(s)`: Converts string s to uppercase
- `lower(s)`: Converts string s to lowercase
- `as_string(value)`: Converts any value to a string

### Time Functions
- `time()`: Returns the current time in seconds since the epoch

### File I/O Functions
- `file(path, mode, [content])`: Read from or write to a file
  - Modes: "r" (read), "w" (write), "a" (append), "e" (exists), "d" (delete)

### Array Functions
- `array(...)`: Creates a new array with the provided elements
- `length(arr)`: Returns the number of elements in the array
- `push(arr, value)`: Adds a value to the end of the array
- `pop(arr)`: Removes and returns the last element from the array
- `get(arr, index)`: Gets the element at the specified index
- `set(arr, index, value)`: Sets the element at the specified index
- `concat(arr1, arr2)`: Combines two arrays
- `join(arr, separator)`: Joins array elements into a string with the given separator

### JSON Functions
- `json_parse(str)`: Parses a JSON string into a Rusty value
- `json_stringify(value)`: Converts a Rusty value to a JSON string

### Regex Functions
- `regex_new(pattern)`: Creates a new regex pattern
- `regex_test(regex, text)`: Tests if a string matches a pattern
- `regex_match_all(regex, text)`: Finds all matches in a string
- `regex_replace_all(regex, text, replacement)`: Replaces all occurrences of a pattern
- `regex_split(regex, text)`: Splits a string by a pattern
- `regex_capture(regex, text)`: Gets an array of capture groups
- `regex_is_valid(pattern)`: Checks if a pattern is valid
- `regex_escape(text)`: Escapes a string for use in a regex pattern

## Operators

### Arithmetic Operators
- `+`: Addition (numbers) or concatenation (strings)
- `-`: Subtraction
- `*`: Multiplication
- `/`: Division

### Comparison Operators
- `==`: Equal to
- `!=`: Not equal to
- `>`: Greater than
- `>=`: Greater than or equal to
- `<`: Less than
- `<=`: Less than or equal to

### Logical Operators
- `&&`: Logical AND
- `||`: Logical OR
- `!`: Logical NOT

### Assignment Operator
- `=`: Assigns a value to a variable

## Evaluation Rules

### Truthiness

In conditional contexts, the following values are considered "falsy":
- `false`
- `nil`

All other values are considered "truthy".

### Operator Precedence

From highest to lowest:
1. Grouping: `()`
2. Function calls: `func()`
3. Unary: `-x`, `!x`
4. Multiplication/Division: `*`, `/`
5. Addition/Subtraction: `+`, `-`
6. Comparison: `<`, `<=`, `>`, `>=`
7. Equality: `==`, `!=`
8. Logical AND: `&&`
9. Logical OR: `||`
10. Assignment: `=`

## Example Program

```
// Calculate factorial using recursion
fun factorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

var n = 5;
print "Factorial of " + as_string(n) + " is " + as_string(factorial(n));

// Use arrays
var numbers = array(1, 2, 3, 4, 5);
var sum = 0;
var i = 0;
while (i < length(numbers)) {
    sum = sum + get(numbers, i);
    i = i + 1;
}
print "Sum of array elements: " + as_string(sum);

// Use regex
var pattern = regex_new("[0-9]+");
var text = "The answer is 42";
var matches = regex_match_all(pattern, text);
if (length(matches) > 0) {
    print "Found number: " + get(matches, 0);
}
```
