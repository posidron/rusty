use crate::interpreter::Value;
use std::iter::Peekable;
use std::str::Chars;

/// Parse JSON string to Rusty value: json_parse(string)
pub fn json_parse(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(json_str) => {
            parse_json_text(json_str)
        },
        _ => Err("json_parse: argument must be a string".to_string()),
    }
}

/// Convert Rusty value to JSON string: json_stringify(value)
pub fn json_stringify(args: Vec<Value>) -> Result<Value, String> {
    let json_str = value_to_json_string(&args[0])?;
    Ok(Value::String(json_str))
}

/// Parse a JSON string into a Rusty Value
fn parse_json_text(json_str: &str) -> Result<Value, String> {
    // Skip whitespace
    let mut chars = json_str.trim().chars().peekable();

    // Parse the value
    parse_json_value(&mut chars)
}

/// Parse a JSON value from a character iterator
fn parse_json_value(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    match chars.peek() {
        Some('{') => parse_json_object(chars),
        Some('[') => parse_json_array(chars),
        Some('"') => parse_json_string(chars),
        Some('t') => parse_json_true(chars),
        Some('f') => parse_json_false(chars),
        Some('n') => parse_json_null(chars),
        Some(c) if c.is_digit(10) || *c == '-' => parse_json_number(chars),
        Some(c) => Err(format!("Unexpected character in JSON: {}", c)),
        None => Err("Unexpected end of JSON input".to_string()),
    }
}

/// Parse a JSON object: {"key": value, ...}
fn parse_json_object(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    // Consume the opening brace
    chars.next();

    // Create a temporary array to hold key-value pairs
    let mut pairs = Vec::new();

    // Skip whitespace
    while let Some(c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        break;
    }

    // Check for empty object
    if let Some('}') = chars.peek() {
        chars.next();
        // Return empty object as empty array in Rusty
        return Ok(Value::Array(Vec::new()));
    }

    loop {
        // Parse key (must be a string)
        if let Some('"') = chars.peek() {
            let key = parse_json_string(chars)?;
            if let Value::String(key_str) = key {
                // Skip whitespace
                while let Some(c) = chars.peek() {
                    if c.is_whitespace() {
                        chars.next();
                        continue;
                    }
                    break;
                }

                // Expect colon
                if let Some(':') = chars.peek() {
                    chars.next();
                } else {
                    return Err("Expected ':' in JSON object".to_string());
                }

                // Skip whitespace
                while let Some(c) = chars.peek() {
                    if c.is_whitespace() {
                        chars.next();
                        continue;
                    }
                    break;
                }

                // Parse value
                let value = parse_json_value(chars)?;

                // Add key-value pair to array
                pairs.push(Value::Array(vec![Value::String(key_str), value]));

                // Skip whitespace
                while let Some(c) = chars.peek() {
                    if c.is_whitespace() {
                        chars.next();
                        continue;
                    }
                    break;
                }

                // Check for comma or closing brace
                match chars.peek() {
                    Some(',') => {
                        chars.next();
                        // Skip whitespace
                        while let Some(c) = chars.peek() {
                            if c.is_whitespace() {
                                chars.next();
                                continue;
                            }
                            break;
                        }
                    },
                    Some('}') => {
                        chars.next();
                        // Return object as array of key-value pairs
                        return Ok(Value::Array(pairs));
                    },
                    _ => return Err("Expected ',' or '}' in JSON object".to_string()),
                }
            } else {
                return Err("Expected string key in JSON object".to_string());
            }
        } else {
            return Err("Expected string key in JSON object".to_string());
        }
    }
}

/// Parse a JSON array: [value, ...]
fn parse_json_array(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    // Consume the opening bracket
    chars.next();

    // Create a vector to hold values
    let mut values = Vec::new();

    // Skip whitespace
    while let Some(c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        break;
    }

    // Check for empty array
    if let Some(']') = chars.peek() {
        chars.next();
        return Ok(Value::Array(Vec::new()));
    }

    loop {
        // Parse value
        let value = parse_json_value(chars)?;
        values.push(value);

        // Skip whitespace
        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                continue;
            }
            break;
        }

        // Check for comma or closing bracket
        match chars.peek() {
            Some(',') => {
                chars.next();
                // Skip whitespace
                while let Some(c) = chars.peek() {
                    if c.is_whitespace() {
                        chars.next();
                        continue;
                    }
                    break;
                }
            },
            Some(']') => {
                chars.next();
                return Ok(Value::Array(values));
            },
            _ => return Err("Expected ',' or ']' in JSON array".to_string()),
        }
    }
}

/// Parse a JSON string: "value"
fn parse_json_string(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    // Consume the opening quote
    chars.next();

    let mut result = String::new();

    loop {
        match chars.next() {
            Some('"') => return Ok(Value::String(result)),
            Some('\\') => {
                // Handle escape sequences
                match chars.next() {
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('/') => result.push('/'),
                    Some('b') => result.push('\u{0008}'),
                    Some('f') => result.push('\u{000C}'),
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some('u') => {
                        // Unicode escape sequence (e.g., \u00A9)
                        let mut code = String::new();
                        for _ in 0..4 {
                            if let Some(c) = chars.next() {
                                code.push(c);
                            } else {
                                return Err("Incomplete Unicode escape sequence in JSON string".to_string());
                            }
                        }

                        // Parse the hexadecimal code
                        if let Ok(code_point) = u32::from_str_radix(&code, 16) {
                            if let Some(c) = std::char::from_u32(code_point) {
                                result.push(c);
                            } else {
                                return Err(format!("Invalid Unicode code point in JSON string: \\u{}", code));
                            }
                        } else {
                            return Err(format!("Invalid Unicode escape sequence in JSON string: \\u{}", code));
                        }
                    },
                    Some(c) => return Err(format!("Invalid escape sequence in JSON string: \\{}", c)),
                    None => return Err("Unexpected end of JSON string".to_string()),
                }
            },
            Some(c) => result.push(c),
            None => return Err("Unterminated JSON string".to_string()),
        }
    }
}

/// Parse JSON true literal
fn parse_json_true(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    // Check for "true"
    if chars.next() == Some('t') &&
       chars.next() == Some('r') &&
       chars.next() == Some('u') &&
       chars.next() == Some('e') {
        Ok(Value::Boolean(true))
    } else {
        Err("Invalid JSON literal".to_string())
    }
}

/// Parse JSON false literal
fn parse_json_false(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    // Check for "false"
    if chars.next() == Some('f') &&
       chars.next() == Some('a') &&
       chars.next() == Some('l') &&
       chars.next() == Some('s') &&
       chars.next() == Some('e') {
        Ok(Value::Boolean(false))
    } else {
        Err("Invalid JSON literal".to_string())
    }
}

/// Parse JSON null literal
fn parse_json_null(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    // Check for "null"
    if chars.next() == Some('n') &&
       chars.next() == Some('u') &&
       chars.next() == Some('l') &&
       chars.next() == Some('l') {
        Ok(Value::Nil)
    } else {
        Err("Invalid JSON literal".to_string())
    }
}

/// Parse a JSON number
fn parse_json_number(chars: &mut Peekable<Chars>) -> Result<Value, String> {
    let mut number_str = String::new();

    // Handle negative sign
    if let Some('-') = chars.peek() {
        number_str.push(chars.next().unwrap());
    }

    // Parse integer part
    while let Some(c) = chars.peek() {
        if c.is_digit(10) {
            number_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    // Parse fractional part
    if let Some('.') = chars.peek() {
        number_str.push(chars.next().unwrap());

        let mut has_fraction = false;
        while let Some(c) = chars.peek() {
            if c.is_digit(10) {
                number_str.push(chars.next().unwrap());
                has_fraction = true;
            } else {
                break;
            }
        }

        if !has_fraction {
            return Err("Invalid JSON number format".to_string());
        }
    }

    // Parse exponent part
    if let Some(c) = chars.peek() {
        if *c == 'e' || *c == 'E' {
            number_str.push(chars.next().unwrap());

            // Handle exponent sign
            if let Some(c) = chars.peek() {
                if *c == '+' || *c == '-' {
                    number_str.push(chars.next().unwrap());
                }
            }

            let mut has_exponent = false;
            while let Some(c) = chars.peek() {
                if c.is_digit(10) {
                    number_str.push(chars.next().unwrap());
                    has_exponent = true;
                } else {
                    break;
                }
            }

            if !has_exponent {
                return Err("Invalid JSON number format".to_string());
            }
        }
    }

    // Parse the number string
    match number_str.parse::<f64>() {
        Ok(number) => Ok(Value::Number(number)),
        Err(_) => Err(format!("Invalid JSON number: {}", number_str)),
    }
}

/// Convert a Rusty Value to a JSON string
fn value_to_json_string(value: &Value) -> Result<String, String> {
    match value {
        Value::Number(n) => Ok(n.to_string()),
        Value::String(s) => {
            // Escape special characters
            let mut result = String::with_capacity(s.len() + 2);
            result.push('"');

            for c in s.chars() {
                match c {
                    '"' => result.push_str("\\\""),
                    '\\' => result.push_str("\\\\"),
                    '\u{0008}' => result.push_str("\\b"),
                    '\u{000C}' => result.push_str("\\f"),
                    '\n' => result.push_str("\\n"),
                    '\r' => result.push_str("\\r"),
                    '\t' => result.push_str("\\t"),
                    c if c.is_control() => {
                        // Format control characters as \uXXXX
                        result.push_str(&format!("\\u{:04x}", c as u32));
                    },
                    _ => result.push(c),
                }
            }

            result.push('"');
            Ok(result)
        },
        Value::Boolean(b) => Ok(b.to_string()),
        Value::Nil => Ok("null".to_string()),
        Value::Array(elements) => {
            // If first element is an array of [key, value] pairs, treat as object
            if !elements.is_empty() && elements.iter().all(|e| {
                if let Value::Array(pair) = e {
                    pair.len() == 2 && matches!(pair[0], Value::String(_))
                } else {
                    false
                }
            }) {
                // Convert to JSON object
                let mut result = String::from("{");

                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }

                    if let Value::Array(pair) = element {
                        if let Value::String(key) = &pair[0] {
                            // Add key
                            result.push_str(&value_to_json_string(&Value::String(key.clone()))?);
                            result.push_str(": ");

                            // Add value
                            result.push_str(&value_to_json_string(&pair[1])?);
                        }
                    }
                }

                result.push('}');
                Ok(result)
            } else {
                // Convert to JSON array
                let mut result = String::from("[");

                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }

                    result.push_str(&value_to_json_string(element)?);
                }

                result.push(']');
                Ok(result)
            }
        },
        Value::Function(_) => {
            // Functions cannot be represented in JSON, so convert to null
            Ok("null".to_string())
        },
        Value::NativeFunction(_) => {
            // Native functions cannot be represented in JSON, so convert to null
            Ok("null".to_string())
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_primitives() {
        // Test parsing simple values
        let test_cases = vec![
            ("42", Value::Number(42.0)),
            ("\"hello\"", Value::String("hello".to_string())),
            ("true", Value::Boolean(true)),
            ("false", Value::Boolean(false)),
            ("null", Value::Nil),
        ];

        for (json, expected) in test_cases {
            let args = vec![Value::String(json.to_string())];
            let result = json_parse(args).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_json_parse_array() {
        let args = vec![Value::String("[1, 2, 3]".to_string())];
        let result = json_parse(args).unwrap();

        if let Value::Array(elements) = result {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Value::Number(1.0));
            assert_eq!(elements[1], Value::Number(2.0));
            assert_eq!(elements[2], Value::Number(3.0));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_json_stringify() {
        // Test stringifying simple values
        let num_args = vec![Value::Number(42.0)];
        let num_result = json_stringify(num_args).unwrap();
        assert_eq!(num_result, Value::String("42".to_string()));

        let str_args = vec![Value::String("hello".to_string())];
        let str_result = json_stringify(str_args).unwrap();
        assert_eq!(str_result, Value::String("\"hello\"".to_string()));

        // Test stringifying array
        let arr_args = vec![Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ])];
        let arr_result = json_stringify(arr_args).unwrap();
        assert_eq!(arr_result, Value::String("[1, 2, 3]".to_string()));
    }
}
