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
                Err(format!("get: index {} out of bounds for array of length {}", idx, elements.len()))
            }
        },
        (Value::Array(_), _) => Err("get: second argument must be a number".to_string()),
        _ => Err("get: first argument must be an array".to_string()),
    }
}

/// Set element at index: set(array, index, value)
pub fn set(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1], &args[2]) {
        (Value::Array(elements), Value::Number(index), value) => {
            let idx = *index as usize;
            let mut new_elements = elements.clone();
            if idx < new_elements.len() {
                new_elements[idx] = value.clone();
                Ok(Value::Array(new_elements))
            } else {
                Err(format!("set: index {} out of bounds for array of length {}", idx, new_elements.len()))
            }
        },
        (Value::Array(_), _, _) => Err("set: second argument must be a number".to_string()),
        _ => Err("set: first argument must be an array".to_string()),
    }
}

/// Concatenate arrays: concat(array1, array2)
pub fn concat(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Array(arr1), Value::Array(arr2)) => {
            let mut result = arr1.clone();
            result.extend(arr2.iter().cloned());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let args = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
        let result = array(args).unwrap();
        if let Value::Array(elements) = result {
            assert_eq!(elements.len(), 3);
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_array_operations() {
        // Create test array
        let empty_args = vec![];
        let mut arr = array(empty_args).unwrap();

        // Test push
        if let Value::Array(elements) = &arr {
            assert_eq!(elements.len(), 0);
        }

        let push_args = vec![arr.clone(), Value::Number(1.0)];
        arr = push(push_args).unwrap();

        if let Value::Array(elements) = &arr {
            assert_eq!(elements.len(), 1);
        }

        // Test get
        let get_args = vec![arr.clone(), Value::Number(0.0)];
        let get_result = get(get_args).unwrap();
        if let Value::Number(n) = get_result {
            assert_eq!(n, 1.0);
        }
    }
}
