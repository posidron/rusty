use crate::interpreter::Value;

/// Get string length
pub fn len(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        Value::Array(a) => Ok(Value::Number(a.len() as f64)),
        Value::Object(_) => Ok(Value::Number(0.0)), // Temporary implementation
        Value::Namespace(_, _) => Ok(Value::Number(0.0)), // Temporary implementation
        _ => Err("len: argument must be a string or array".to_string()),
    }
}

/// Convert to uppercase
pub fn upper(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("upper: expected 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        Value::Object(_) => Err("upper: cannot convert object to uppercase".to_string()),
        Value::Namespace(_, _) => Err("upper: cannot convert namespace to uppercase".to_string()),
        _ => Err("upper: argument must be a string".to_string()),
    }
}

/// Convert to lowercase
pub fn lower(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("lower: expected 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        Value::Object(_) => Err("lower: cannot convert object to lowercase".to_string()),
        Value::Namespace(_, _) => Err("lower: cannot convert namespace to lowercase".to_string()),
        _ => Err("lower: argument must be a string".to_string()),
    }
}

/// Convert value to string
pub fn as_string(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("as_string: expected 1 argument, got {}", args.len()));
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Number(n) => {
            let text = n.to_string();
            // Remove trailing ".0" for whole numbers
            if text.ends_with(".0") {
                Ok(Value::String(text[..text.len() - 2].to_string()))
            } else {
                Ok(Value::String(text))
            }
        },
        Value::Boolean(b) => Ok(Value::String(b.to_string())),
        Value::Nil => Ok(Value::String("nil".to_string())),
        Value::Function(f) => Ok(Value::String(format!("<fn {}>", f.name.lexeme))),
        Value::NativeFunction(f) => Ok(Value::String(format!("<native fn {}>", f.name))),
        Value::Array(a) => {
            // Convert array to a simple string representation
            let mut result = String::from("[");
            for (i, element) in a.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }

                // Simple representation for nested elements
                match element {
                    Value::Number(n) => result.push_str(&n.to_string()),
                    Value::String(s) => result.push_str(s),
                    Value::Boolean(b) => result.push_str(&b.to_string()),
                    Value::Nil => result.push_str("nil"),
                    Value::Array(_) => result.push_str("[...]"),
                    Value::Object(_) => result.push_str("{...}"), // Simple representation for objects
                    Value::Namespace(name, _) => result.push_str(&format!("[Namespace: {}]", name)), // Simple representation for namespaces
                    Value::Function(_) | Value::NativeFunction(_) => result.push_str("<function>"),
                }
            }
            result.push(']');
            Ok(Value::String(result))
        },
        Value::Object(_) => Ok(Value::String("{object}".to_string())),
        Value::Namespace(name, _) => Ok(Value::String(format!("[Namespace: {}]", name))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let args = vec![Value::String("hello".to_string())];
        let result = len(args).unwrap();
        if let Value::Number(n) = result {
            assert_eq!(n, 5.0);
        } else {
            panic!("Expected number result");
        }
    }

    #[test]
    fn test_upper_lower() {
        let args = vec![Value::String("Hello".to_string())];

        let upper_result = upper(args.clone()).unwrap();
        if let Value::String(s) = upper_result {
            assert_eq!(s, "HELLO");
        } else {
            panic!("Expected string result");
        }

        let lower_result = lower(args).unwrap();
        if let Value::String(s) = lower_result {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected string result");
        }
    }
}
