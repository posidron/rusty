# Rusty Language Standard Library

This document lists all the built-in functions available in the Rusty language standard library.

## Math Functions

| Function | Arguments | Description | Example |
|----------|-----------|-------------|---------|
| `random()` | None | Returns a random floating-point number between 0 and 1 | `var x = random();` |
| `random_range(min, max)` | Two numbers | Returns a random integer between min and max (inclusive) | `var x = random_range(1, 100);` |
| `abs(x)` | One number | Returns the absolute value of x | `var x = abs(-42);` |
| `round(x)` | One number | Rounds x to the nearest integer | `var x = round(3.7);` |
| `floor(x)` | One number | Rounds x down to the nearest integer | `var x = floor(3.7);` |
| `ceil(x)` | One number | Rounds x up to the nearest integer | `var x = ceil(3.2);` |
| `min(a, b)` | Two numbers | Returns the smaller of a and b | `var x = min(5, 10);` |
| `max(a, b)` | Two numbers | Returns the larger of a and b | `var x = max(5, 10);` |

## String Functions

| Function | Arguments | Description | Example |
|----------|-----------|-------------|---------|
| `len(str)` | One string | Returns the length of the string | `var length = len("hello");` |
| `upper(str)` | One string | Converts the string to uppercase | `var upper = upper("hello");` |
| `lower(str)` | One string | Converts the string to lowercase | `var lower = lower("HELLO");` |
| `as_string(value)` | Any value | Converts a value to its string representation | `var str = as_string(42);` |

## Time Functions

| Function | Arguments | Description | Example |
|----------|-----------|-------------|---------|
| `time()` | None | Returns the current time in milliseconds since epoch | `var now = time();` |

## Function Chaining

Functions can be chained together to perform complex operations in a single expression.

Example:
```
// Convert a random number to uppercase string and get its length
var result = len(upper(as_string(round(random() * 1000))));
```

## Error Handling

All standard library functions perform type checking on their arguments and will raise appropriate error messages if invalid arguments are provided.

Examples of errors:
- Passing a string when a number is expected: `abs("not a number")`
- Passing a number when a string is expected: `len(42)`
- Providing too few arguments: `random_range(1)`
- Providing too many arguments: `min(1, 2, 3)`

## Complete Example

```
// Math
var r = random();
var rand_int = random_range(1, 100);
var absolute = abs(-42);
var rounded = round(3.14159);

// Strings
var message = "Hello, World!";
var length = len(message);
var uppercase = upper(message);
var number_as_string = as_string(42);

// Time
var current_time = time();

// Combined example
var num = random_range(1, 100);
var str = as_string(num);
var uppercase_str = upper(str);
print uppercase_str;
```
