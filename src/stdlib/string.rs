use crate::interpreter::Value;

/// Get string length
pub fn len(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err("len: argument must be a string".to_string()),
    }
}

/// Convert to uppercase
pub fn upper(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err("upper: argument must be a string".to_string()),
    }
}

/// Convert to lowercase
pub fn lower(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err("lower: argument must be a string".to_string()),
    }
}

/// Convert value to string
pub fn as_string(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => {
            let text = n.to_string();
            // Remove trailing .0 for whole numbers
            if text.ends_with(".0") {
                Ok(Value::String(text[..text.len() - 2].to_string()))
            } else {
                Ok(Value::String(text))
            }
        },
        Value::String(s) => Ok(Value::String(s.clone())),
        Value::Boolean(b) => Ok(Value::String(b.to_string())),
        Value::Nil => Ok(Value::String("nil".to_string())),
        Value::Function(f) => Ok(Value::String(format!("<fn {}>", f.name.lexeme))),
        Value::NativeFunction(f) => Ok(Value::String(format!("<native fn {}>", f.name))),
        Value::Array(elements) => {
            let mut result = String::from("[");
            for (i, value) in elements.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                // Simple representation for nested elements
                match value {
                    Value::String(s) => result.push_str(&format!("\"{}\"", s)),
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
                    Value::Array(_) => result.push_str("[...]"), // Simple representation for nested arrays
                }
            }
            result.push_str("]");
            Ok(Value::String(result))
        },
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
