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

## File I/O Functions

| Function | Arguments | Description | Example |
|----------|-----------|-------------|---------|
| `file(path, mode, [content])` | Two or three strings | Performs a file operation based on the specified mode | See examples below |

The `mode` parameter can be one of:
- `"r"` - Read: Reads the content of a file as a string
- `"w"` - Write: Writes content to a file (creating it if it doesn't exist)
- `"a"` - Append: Appends content to an existing file (or creates a new file)
- `"e"` - Exists: Checks if a file exists
- `"d"` - Delete: Deletes a file if it exists

Examples:
```
// Check if a file exists
var exists = file("data.txt", "e");  // Returns a boolean

// Read a file
var content = file("data.txt", "r");  // Returns a string

// Write to a file
var success = file("data.txt", "w", "Hello, World!");  // Returns true/false

// Append to a file
var success = file("data.txt", "a", "\nMore content");  // Returns true/false

// Delete a file
var success = file("data.txt", "d");  // Returns true/false
```

## Array Functions

| Function | Arguments | Description | Example |
|----------|-----------|-------------|---------|
| `array([items...])` | Any number of values | Creates a new array with the provided items | `var arr = array(1, 2, 3);` |
| `length(array)` | One array | Returns the number of elements in the array | `var len = length(arr);` |
| `push(array, item)` | Array and any value | Adds an item to the end of the array and returns the new array | `var new_arr = push(arr, 4);` |
| `pop(array)` | One array | Removes and returns the last item from the array | `var last = pop(arr);` |
| `get(array, index)` | Array and number | Returns the item at the specified index | `var first = get(arr, 0);` |
| `set(array, index, value)` | Array, number, and any value | Returns a new array with the item at the specified index replaced | `var updated = set(arr, 1, 99);` |
| `concat(array1, array2)` | Two arrays | Returns a new array with the contents of both arrays | `var combined = concat(arr1, arr2);` |
| `join(array, separator)` | Array and string | Joins array elements into a string with the specified separator | `var str = join(arr, ", ");` |

Examples:
```
// Create an array
var numbers = array(1, 2, 3, 4, 5);

// Get array length
var count = length(numbers);  // 5

// Access elements
var first = get(numbers, 0);  // 1

// Modify array
var modified = set(numbers, 2, 99);  // [1, 2, 99, 4, 5]

// Add elements
var extended = push(numbers, 6);  // [1, 2, 3, 4, 5, 6]

// Remove elements
var last = pop(extended);  // 6

// Combine arrays
var more = array(7, 8, 9);
var combined = concat(numbers, more);  // [1, 2, 3, 4, 5, 7, 8, 9]

// Join elements
var joined = join(numbers, "-");  // "1-2-3-4-5"
```

## JSON Functions

| Function | Arguments | Description | Example |
|----------|-----------|-------------|---------|
| `json_parse(string)` | One string | Parses a JSON string into Rusty values | `var data = json_parse(json_str);` |
| `json_stringify(value)` | Any value | Converts a Rusty value to a JSON string | `var json = json_stringify(data);` |

JSON objects are represented as arrays of key-value pairs in Rusty. Each key-value pair is itself an array containing two elements: the key (string) and the value.

Examples:
```
// Parse JSON string
var data = json_parse("{\"name\": \"John\", \"age\": 30}");

// Access object properties
// Objects are arrays of [key, value] pairs
var first_pair = get(data, 0);  // ["name", "John"]
var key = get(first_pair, 0);   // "name"
var value = get(first_pair, 1); // "John"

// Create an object structure
var person = array(
    array("name", "Alice"),
    array("age", 25),
    array("skills", array("coding", "design"))
);

// Convert to JSON
var json = json_stringify(person);
// Result: '{"name": "Alice", "age": 25, "skills": ["coding", "design"]}'
```

## Regular Expression Functions

| Function | Arguments | Description | Example |
|----------|-----------|-------------|---------|
| `regex_new(pattern)` | One string | Creates a new regular expression pattern | `var regex = regex_new("[0-9]+");` |
| `regex_test(regex, text)` | Regex and string | Tests if a string matches a regex pattern | `var is_match = regex_test(regex, "123");` |
| `regex_match_all(regex, text)` | Regex and string | Finds all matches of a pattern in a string | `var matches = regex_match_all(regex, "abc123def456");` |
| `regex_replace_all(regex, text, replacement)` | Regex, string, and string | Replaces all occurrences of a pattern | `var result = regex_replace_all(regex, "abc123", "X");` |
| `regex_split(regex, text)` | Regex and string | Splits a string by a regex pattern | `var parts = regex_split(regex_new(","), "a,b,c");` |
| `regex_capture(regex, text)` | Regex and string | Gets capture groups from the first match | `var groups = regex_capture(r, "width=100");` |
| `regex_is_valid(pattern)` | One string | Checks if a regex pattern is valid | `var is_valid = regex_is_valid("[0-9]+");` |
| `regex_escape(text)` | One string | Escapes special characters in a string for use in a regex | `var escaped = regex_escape("a.b*c");` |

Examples:
```
// Create a regex pattern
var digit_pattern = regex_new("[0-9]+");

// Test if a string matches the pattern
var is_digits = regex_test(digit_pattern, "123");  // true
var not_digits = regex_test(digit_pattern, "abc"); // false

// Find all matches in a string
var text = "The price is $10.99 for item #42";
var numbers = regex_match_all(digit_pattern, text);
// numbers = ["10", "99", "42"]

// Replace all occurrences
var replaced = regex_replace_all(digit_pattern, text, "XX");
// replaced = "The price is $XX.XX for item #XX"

// Split a string
var csv = "apple,orange,banana";
var fruits = regex_split(regex_new(","), csv);
// fruits = ["apple", "orange", "banana"]

// Capture groups
var key_value_pattern = regex_new("([a-z]+)=([0-9]+)");
var config_text = "width=100 height=200";
var match = regex_capture(key_value_pattern, config_text);
// match = ["width=100", "width", "100"]

// Escape special characters
var escaped = regex_escape("Hello (.*+?) world!");
// escaped = "Hello \\(\\.\\*\\+\\?\\) world!"
```

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
- Using an invalid file mode: `file("data.txt", "x")`

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

// File I/O
var exists = file("example.txt", "e");
if (!exists) {
    file("example.txt", "w", "Hello from Rusty!");
}
var content = file("example.txt", "r");
file("example.txt", "a", "\nMore content added!");
var updated_content = file("example.txt", "r");
print updated_content;
file("example.txt", "d");  // Delete file when done

// Arrays
var numbers = array(1, 2, 3, 4, 5);
var first = get(numbers, 0);
var new_array = push(numbers, 6);
var joined = join(numbers, ", ");

// JSON
var person = json_parse('{"name": "John", "age": 30}');
var json_str = json_stringify(numbers);

// Regex
var pattern = regex_new("[0-9]+");
var has_digits = regex_test(pattern, "abc123");
var matches = regex_match_all(pattern, "User123 has 42 points");
var sanitized = regex_replace_all(pattern, "Password123", "***");

// Combined example
var num = random_range(1, 100);
var str = as_string(num);
var uppercase_str = upper(str);
print uppercase_str;
```
