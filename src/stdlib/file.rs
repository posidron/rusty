use crate::interpreter::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Unified file function for all operations
pub fn file(args: Vec<Value>) -> Result<Value, String> {
    // Third argument (content) is optional, defaulting to nil
    let content = if args.len() > 2 { &args[2] } else { &Value::Nil };

    match (&args[0], &args[1]) {
        (Value::String(filepath), Value::String(mode)) => {
            match mode.as_str() {
                // Read mode: file(path, "r") -> string content
                "r" => {
                    match fs::read_to_string(filepath) {
                        Ok(content) => Ok(Value::String(content)),
                        Err(err) => Err(format!("file: failed to read '{}': {}", filepath, err)),
                    }
                },

                // Write mode: file(path, "w", content) -> boolean success
                "w" => {
                    if let Value::String(content_str) = content {
                        // Ensure parent directory exists
                        if let Some(parent) = Path::new(filepath).parent() {
                            if !parent.exists() {
                                if let Err(err) = fs::create_dir_all(parent) {
                                    return Err(format!("file: failed to create directories for '{}': {}", filepath, err));
                                }
                            }
                        }

                        match fs::write(filepath, content_str) {
                            Ok(_) => Ok(Value::Boolean(true)),
                            Err(err) => Err(format!("file: failed to write to '{}': {}", filepath, err)),
                        }
                    } else {
                        Err("file: in write mode, content must be a string".to_string())
                    }
                },

                // Append mode: file(path, "a", content) -> boolean success
                "a" => {
                    if let Value::String(content_str) = content {
                        // Ensure parent directory exists
                        if let Some(parent) = Path::new(filepath).parent() {
                            if !parent.exists() {
                                if let Err(err) = fs::create_dir_all(parent) {
                                    return Err(format!("file: failed to create directories for '{}': {}", filepath, err));
                                }
                            }
                        }

                        let result = if Path::new(filepath).exists() {
                            // Append to existing file
                            let mut file = match fs::OpenOptions::new().append(true).open(filepath) {
                                Ok(file) => file,
                                Err(err) => return Err(format!("file: failed to open '{}': {}", filepath, err)),
                            };

                            match file.write_all(content_str.as_bytes()) {
                                Ok(_) => Ok(Value::Boolean(true)),
                                Err(err) => Err(format!("file: failed to append to '{}': {}", filepath, err)),
                            }
                        } else {
                            // Create new file
                            match fs::write(filepath, content_str) {
                                Ok(_) => Ok(Value::Boolean(true)),
                                Err(err) => Err(format!("file: failed to create '{}': {}", filepath, err)),
                            }
                        };

                        result
                    } else {
                        Err("file: in append mode, content must be a string".to_string())
                    }
                },

                // Delete mode: file(path, "d") -> boolean success
                "d" => {
                    let path = Path::new(filepath);
                    if !path.exists() {
                        return Ok(Value::Boolean(false));
                    }

                    match fs::remove_file(path) {
                        Ok(_) => Ok(Value::Boolean(true)),
                        Err(err) => Err(format!("file: failed to delete '{}': {}", filepath, err)),
                    }
                },

                // Exists mode: file(path, "e") -> boolean exists
                "e" => {
                    Ok(Value::Boolean(Path::new(filepath).exists()))
                },

                // Invalid mode
                _ => Err(format!("file: invalid mode '{}' (valid modes: 'r', 'w', 'a', 'd', 'e')", mode)),
            }
        },
        (_, Value::String(_)) => Err("file: first argument must be a string filepath".to_string()),
        (Value::String(_), _) => Err("file: second argument must be a string mode".to_string()),
        _ => Err("file: arguments must be (string filepath, string mode, [optional content])".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_file_operations() {
        // Ensure test directory exists
        let test_dir = "test_files";
        let test_path = format!("{}/test_file.txt", test_dir);

        if !Path::new(test_dir).exists() {
            fs::create_dir_all(test_dir).expect("Failed to create test directory");
        }

        // Test write
        let write_args = vec![
            Value::String(test_path.clone()),
            Value::String("w".to_string()),
            Value::String("Test content".to_string()),
        ];
        let write_result = file(write_args).unwrap();
        assert!(matches!(write_result, Value::Boolean(true)));

        // Test exists (should be true)
        let exists_args = vec![
            Value::String(test_path.clone()),
            Value::String("e".to_string()),
        ];
        let exists_result = file(exists_args).unwrap();
        assert!(matches!(exists_result, Value::Boolean(true)));

        // Test read
        let read_args = vec![
            Value::String(test_path.clone()),
            Value::String("r".to_string()),
        ];
        let read_result = file(read_args).unwrap();
        if let Value::String(content) = read_result {
            assert_eq!(content, "Test content");
        } else {
            panic!("Expected string result");
        }

        // Test delete
        let delete_args = vec![
            Value::String(test_path.clone()),
            Value::String("d".to_string()),
        ];
        let delete_result = file(delete_args).unwrap();
        assert!(matches!(delete_result, Value::Boolean(true)));

        // Cleanup
        if Path::new(&test_path).exists() {
            fs::remove_file(&test_path).ok();
        }
    }
}
