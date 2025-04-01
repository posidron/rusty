use crate::interpreter::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Read a file and return its contents as a string
pub fn read(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("read: expected 1 argument, got {}", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("read: expected a string filepath".to_string()),
    };

    match fs::read_to_string(path) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(format!("read: failed to read file '{}': {}", path, e)),
    }
}

/// Write content to a file
pub fn write(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("write: expected 2 arguments, got {}", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("write: expected a string filepath".to_string()),
    };

    let content = match &args[1] {
        Value::String(s) => s,
        _ => return Err("write: expected a string content".to_string()),
    };

    match fs::write(path, content) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(format!("write: failed to write to file '{}': {}", path, e)),
    }
}

/// Append content to a file
pub fn append(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("append: expected 2 arguments, got {}", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("append: expected a string filepath".to_string()),
    };

    let content = match &args[1] {
        Value::String(s) => s,
        _ => return Err("append: expected a string content".to_string()),
    };

    let result = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .and_then(|mut file| {
            file.write_all(content.as_bytes())?;
            Ok(())
        });

    match result {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(format!("append: failed to append to file '{}': {}", path, e)),
    }
}

/// Check if a file exists
pub fn exists(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("exists: expected 1 argument, got {}", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("exists: expected a string filepath".to_string()),
    };

    Ok(Value::Boolean(std::path::Path::new(path).exists()))
}

/// Delete a file
pub fn delete(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("delete: expected 1 argument, got {}", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("delete: expected a string filepath".to_string()),
    };

    if !std::path::Path::new(path).exists() {
        return Ok(Value::Boolean(false));
    }

    match fs::remove_file(path) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(e) => Err(format!("delete: failed to delete file '{}': {}", path, e)),
    }
}

/// The unified file function that was used previously
/// This maintains backward compatibility
pub fn file(args: Vec<Value>) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("file: requires at least 2 arguments (path, mode)".to_string());
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err("file: first argument must be a string filepath".to_string()),
    };

    let mode = match &args[1] {
        Value::String(s) => s,
        _ => return Err("file: second argument must be a string mode".to_string()),
    };

    match mode.as_str() {
        "r" => read(vec![args[0].clone()]),
        "w" => {
            if args.len() < 3 {
                return Err("file: write mode requires content argument".to_string());
            }
            write(vec![args[0].clone(), args[2].clone()])
        }
        "a" => {
            if args.len() < 3 {
                return Err("file: append mode requires content argument".to_string());
            }
            append(vec![args[0].clone(), args[2].clone()])
        }
        "e" => exists(vec![args[0].clone()]),
        "d" => delete(vec![args[0].clone()]),
        _ => Err(format!("file: invalid mode '{}', expected 'r', 'w', 'a', 'e', or 'd'", mode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Read;

    #[test]
    fn test_file_operations() {
        // Create a temporary file
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_str().unwrap().to_string();

        // Test write
        let write_args = vec![
            Value::String(path.clone()),
            Value::String("Hello, world!".to_string()),
        ];
        let write_result = write(write_args).unwrap();
        assert_eq!(write_result, Value::Boolean(true));

        // Test read
        let read_args = vec![Value::String(path.clone())];
        let read_result = read(read_args).unwrap();
        if let Value::String(content) = read_result {
            assert_eq!(content, "Hello, world!");
        } else {
            panic!("Expected string result");
        }

        // Test append
        let append_args = vec![
            Value::String(path.clone()),
            Value::String(" Appended text.".to_string()),
        ];
        let append_result = append(append_args).unwrap();
        assert_eq!(append_result, Value::Boolean(true));

        // Test read after append
        let read_args = vec![Value::String(path.clone())];
        let read_result = read(read_args).unwrap();
        if let Value::String(content) = read_result {
            assert_eq!(content, "Hello, world! Appended text.");
        } else {
            panic!("Expected string result");
        }

        // Test exists
        let exists_args = vec![Value::String(path.clone())];
        let exists_result = exists(exists_args).unwrap();
        assert_eq!(exists_result, Value::Boolean(true));

        // Test delete
        let delete_args = vec![Value::String(path.clone())];
        let delete_result = delete(delete_args).unwrap();
        assert_eq!(delete_result, Value::Boolean(true));

        // Test exists after delete
        let exists_args = vec![Value::String(path.clone())];
        let exists_result = exists(exists_args).unwrap();
        assert_eq!(exists_result, Value::Boolean(false));
    }
}
