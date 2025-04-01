use crate::interpreter::Value;

/// Create a new array: array(item1, item2, ...)
pub fn array(args: Vec<Value>) -> Result<Value, String> {
    // We allow variable number of arguments here
    Ok(Value::Array(args))
}

/// Get array or string length: length(array)
pub fn length(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(elements) => Ok(Value::Number(elements.len() as f64)),
        Value::String(s) => Ok(Value::Number(s.len() as f64)), // Also works for strings for compatibility
        _ => Err("length: argument must be an array or string".to_string()),
    }
}

/// Push item to array: push(array, item)
pub fn push(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(elements) => {
            let mut new_elements = elements.clone();
            new_elements.push(args[1].clone());
            Ok(Value::Array(new_elements))
        },
        _ => Err("push: first argument must be an array".to_string()),
    }
}

/// Pop item from array: pop(array)
pub fn pop(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(elements) => {
            let mut new_elements = elements.clone();
            if new_elements.is_empty() {
                Err("pop: cannot pop from empty array".to_string())
            } else {
                let popped = new_elements.pop().unwrap();
                Ok(popped)
            }
        },
        _ => Err("pop: argument must be an array".to_string()),
    }
}

/// Get element at index: get(array, index)
pub fn get(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Array(elements), Value::Number(index)) => {
            let idx = *index as usize;
            if idx < elements.len() {
                Ok(elements[idx].clone())
            } else {
                Err(format!("get: index {} out of bounds (array length: {})", idx, elements.len()))
            }
        },
        (Value::String(s), Value::Number(index)) => {
            let idx = *index as usize;
            if idx < s.len() {
                // Extract a single character
                let c = s.chars().nth(idx).unwrap();
                Ok(Value::String(c.to_string()))
            } else {
                Err(format!("get: index {} out of bounds (string length: {})", idx, s.len()))
            }
        },
        (Value::Array(_), _) => Err("get: second argument must be a number (index)".to_string()),
        (Value::String(_), _) => Err("get: second argument must be a number (index)".to_string()),
        _ => Err("get: first argument must be an array or string".to_string()),
    }
}

/// Set element at index: set(array, index, value)
pub fn set(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Array(elements), Value::Number(index)) => {
            let idx = *index as usize;
            if idx < elements.len() {
                let mut new_elements = elements.clone();
                new_elements[idx] = args[2].clone();
                Ok(Value::Array(new_elements))
            } else {
                Err(format!("set: index {} out of bounds (array length: {})", idx, elements.len()))
            }
        },
        (Value::String(s), Value::Number(index)) => {
            let idx = *index as usize;
            if idx >= s.len() {
                return Err(format!("set: index {} out of bounds (string length: {})", idx, s.len()));
            }

            match &args[2] {
                Value::String(new_char) => {
                    if new_char.len() != 1 {
                        return Err("set: replacement must be a single character".to_string());
                    }

                    // Convert the string to a vector of chars
                    let mut chars: Vec<char> = s.chars().collect();
                    chars[idx] = new_char.chars().next().unwrap();

                    // Convert back to string
                    Ok(Value::String(chars.into_iter().collect()))
                },
                _ => Err("set: third argument must be a string (character)".to_string()),
            }
        },
        (Value::Array(_), _) => Err("set: second argument must be a number (index)".to_string()),
        (Value::String(_), _) => Err("set: second argument must be a number (index)".to_string()),
        _ => Err("set: first argument must be an array or string".to_string()),
    }
}

/// Access element with bracket notation: index(array, index)
/// This provides a more intuitive syntax through the function call mechanism:
/// array[5] becomes "index(array, 5)" in the parsed tree
pub fn index(args: Vec<Value>) -> Result<Value, String> {
    // Simply call get with the same arguments
    get(args)
}

/// Set element with bracket notation: index_set(array, index, value)
/// This provides a more intuitive syntax:
/// array[5] = "hello" becomes "index_set(array, 5, "hello")" in the parsed tree
pub fn index_set(args: Vec<Value>) -> Result<Value, String> {
    // Simply call set with the same arguments
    set(args)
}

/// Concatenate two arrays: concat(array1, array2)
pub fn concat(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Array(elements1), Value::Array(elements2)) => {
            let mut result = elements1.clone();
            result.extend(elements2.clone());
            Ok(Value::Array(result))
        },
        (Value::Array(_), _) => Err("concat: second argument must be an array".to_string()),
        _ => Err("concat: first argument must be an array".to_string()),
    }
}

/// Join array elements into a string: join(array, separator)
pub fn join(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Array(elements), Value::String(separator)) => {
            let mut result = String::new();
            for (i, value) in elements.iter().enumerate() {
                if i > 0 {
                    result.push_str(separator);
                }
                match value {
                    Value::String(s) => result.push_str(s),
                    Value::Number(n) => {
                        let text = n.to_string();
                        if text.ends_with(".0") {
                            result.push_str(&text[..text.len() - 2]);
                        } else {
                            result.push_str(&text);
                        }
                    },
                    Value::Boolean(b) => result.push_str(&b.to_string()),
                    Value::Nil => result.push_str("nil"),
                    Value::Function(f) => result.push_str(&format!("<fn {}>", f.name.lexeme)),
                    Value::NativeFunction(f) => result.push_str(&format!("<native fn {}>", f.name)),
                    Value::Array(_) => result.push_str("[array]"),
                }
            }
            Ok(Value::String(result))
        },
        (Value::Array(_), _) => Err("join: second argument must be a string".to_string()),
        _ => Err("join: first argument must be an array".to_string()),
    }
}

/// Slice an array or string: slice(array/string, start, [length])
/// Returns a new array/string with elements from start to start+length-1
pub fn slice(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err("slice: requires 2 or 3 arguments".to_string());
    }

    let length_arg = if args.len() == 3 {
        match &args[2] {
            Value::Number(n) => Some(*n as usize),
            _ => return Err("slice: third argument (length) must be a number".to_string()),
        }
    } else {
        None
    };

    match (&args[0], &args[1]) {
        (Value::Array(elements), Value::Number(start_idx)) => {
            let start = *start_idx as usize;
            if start > elements.len() {
                return Err(format!("slice: start index {} out of bounds (array length: {})", start, elements.len()));
            }

            let length = length_arg.unwrap_or(elements.len() - start);
            let end = std::cmp::min(start + length, elements.len());

            let result = elements[start..end].to_vec();
            Ok(Value::Array(result))
        },
        (Value::String(s), Value::Number(start_idx)) => {
            let start = *start_idx as usize;
            if start > s.len() {
                return Err(format!("slice: start index {} out of bounds (string length: {})", start, s.len()));
            }

            // Convert string to chars to properly handle multi-byte characters
            let chars: Vec<char> = s.chars().collect();
            let length = length_arg.unwrap_or(chars.len() - start);
            let end = std::cmp::min(start + length, chars.len());

            let result: String = chars[start..end].iter().collect();
            Ok(Value::String(result))
        },
        _ => Err("slice: first argument must be an array or string".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let args = vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ];
        let result = array(args).unwrap();

        if let Value::Array(elements) = result {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[0], Value::Number(1.0)));
            assert!(matches!(elements[1], Value::Number(2.0)));
            assert!(matches!(elements[2], Value::Number(3.0)));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_length() {
        // Test array length
        let args = vec![
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
        ];
        let result = length(args).unwrap();
        assert!(matches!(result, Value::Number(3.0)));

        // Test string length
        let args = vec![
            Value::String("hello".to_string()),
        ];
        let result = length(args).unwrap();
        assert!(matches!(result, Value::Number(5.0)));
    }

    #[test]
    fn test_push() {
        let args = vec![
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
            ]),
            Value::Number(3.0),
        ];
        let result = push(args).unwrap();

        if let Value::Array(elements) = result {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[2], Value::Number(3.0)));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_pop() {
        let args = vec![
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
        ];
        let result = pop(args).unwrap();
        assert!(matches!(result, Value::Number(3.0)));
    }

    #[test]
    fn test_get() {
        // Test array get
        let args = vec![
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
            Value::Number(1.0),
        ];
        let result = get(args).unwrap();
        assert!(matches!(result, Value::Number(2.0)));

        // Test string get
        let args = vec![
            Value::String("hello".to_string()),
            Value::Number(1.0),
        ];
        let result = get(args).unwrap();
        assert!(matches!(result, Value::String(s) if s == "e"));
    }

    #[test]
    fn test_set() {
        // Test array set
        let args = vec![
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
            Value::Number(1.0),
            Value::Number(99.0),
        ];
        let result = set(args).unwrap();

        if let Value::Array(elements) = result {
            assert!(matches!(elements[1], Value::Number(99.0)));
        } else {
            panic!("Expected array result");
        }

        // Test string set
        let args = vec![
            Value::String("hello".to_string()),
            Value::Number(1.0),
            Value::String("a".to_string()),
        ];
        let result = set(args).unwrap();

        if let Value::String(s) = result {
            assert_eq!(s, "hallo");
        } else {
            panic!("Expected string result");
        }
    }

    #[test]
    fn test_slice() {
        // Test array slice
        let args = vec![
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
                Value::Number(4.0),
            ]),
            Value::Number(1.0),
            Value::Number(2.0),
        ];
        let result = slice(args).unwrap();

        if let Value::Array(elements) = result {
            assert_eq!(elements.len(), 2);
            assert!(matches!(elements[0], Value::Number(2.0)));
            assert!(matches!(elements[1], Value::Number(3.0)));
        } else {
            panic!("Expected array result");
        }

        // Test string slice
        let args = vec![
            Value::String("hello".to_string()),
            Value::Number(1.0),
            Value::Number(3.0),
        ];
        let result = slice(args).unwrap();

        if let Value::String(s) = result {
            assert_eq!(s, "ell");
        } else {
            panic!("Expected string result");
        }
    }
}
