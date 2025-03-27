use crate::interpreter::Value;
use regex::Regex;

/// Process the pattern string to handle backslash escaping
/// In the rusty language, \\d is parsed as \d, but Rust's regex needs \\d
/// to represent the digit metacharacter
fn process_pattern(pattern: &str) -> String {
    // Basic implementation - handle common metacharacters manually
    // A proper lexer-based implementation would be more robust
    let mut result = String::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.peek() {
                // These are common regex metacharacters that need special handling
                match next {
                    'd' | 'D' | 'w' | 'W' | 's' | 'S' | 'b' | 'B' |
                    'n' | 'r' | 't' | '0' | 'A' | 'Z' | 'z' | 'G' |
                    'p' | 'P' => {
                        // Keep the backslash for these metacharacters
                        result.push('\\');
                        result.push(*next);
                        chars.next(); // Consume the next character
                    },
                    // Handle hex and unicode escapes
                    'x' | 'u' | 'U' => {
                        result.push('\\');
                        result.push(*next);
                        chars.next(); // Consume the next character
                    },
                    // For any other character after a backslash, just copy both
                    _ => {
                        result.push('\\');
                        result.push(*next);
                        chars.next(); // Consume the next character
                    }
                }
            } else {
                // Trailing backslash, just copy it
                result.push('\\');
            }
        } else {
            // Not a backslash, just copy the character
            result.push(c);
        }
    }

    result
}

/// Creates a new Regex pattern
///
/// Args:
///     pattern: A string containing the regex pattern
///
/// Returns:
///     A wrapped Regex object represented as an array containing the pattern and compiled regex
pub fn regex_new(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("regex_new expects 1 argument".to_string());
    }

    let pattern = match &args[0] {
        Value::String(s) => s,
        _ => return Err("regex_new expects a string pattern".to_string()),
    };

    // Process the pattern to handle escaping properly
    let processed_pattern = process_pattern(pattern);

    // Compile the regex pattern with the processed pattern
    match Regex::new(&processed_pattern) {
        Ok(_) => {
            // Store the original and processed pattern as a special array
            let mut result = Vec::new();
            result.push(Value::String(pattern.clone()));
            result.push(Value::String(format!("__REGEX__{}", processed_pattern)));

            Ok(Value::Array(result))
        },
        Err(e) => Err(format!("Invalid regex pattern: {}", e)),
    }
}

/// Checks if the given value is a regex
fn is_regex(value: &Value) -> bool {
    if let Value::Array(arr) = value {
        if arr.len() == 2 {
            if let Value::String(_) = &arr[0] {
                if let Value::String(marker) = &arr[1] {
                    return marker.starts_with("__REGEX__");
                }
            }
        }
    }
    false
}

/// Gets the processed pattern from a regex object
fn get_processed_pattern(regex_obj: &Value) -> Result<String, String> {
    if !is_regex(regex_obj) {
        return Err("Expected a regex object".to_string());
    }

    if let Value::Array(arr) = regex_obj {
        if let Value::String(marker) = &arr[1] {
            if let Some(pattern) = marker.strip_prefix("__REGEX__") {
                return Ok(pattern.to_string());
            }
        }
    }

    Err("Invalid regex object".to_string())
}

/// Recreates a compiled Regex from a regex object
fn get_compiled_regex(regex_obj: &Value) -> Result<Regex, String> {
    let pattern = get_processed_pattern(regex_obj)?;

    match Regex::new(&pattern) {
        Ok(r) => Ok(r),
        Err(e) => Err(format!("Failed to recreate regex: {}", e)),
    }
}

/// Tests if a string matches a regex pattern
///
/// Args:
///     regex: A regex object created with regex_new
///     text: The string to test against the pattern
///
/// Returns:
///     true if the pattern matches, false otherwise
pub fn regex_test(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("regex_test expects 2 arguments".to_string());
    }

    let regex_obj = &args[0];
    if !is_regex(regex_obj) {
        return Err("First argument must be a regex object created with regex_new".to_string());
    }

    let text = match &args[1] {
        Value::String(s) => s,
        _ => return Err("Second argument must be a string".to_string()),
    };

    let regex = get_compiled_regex(regex_obj)?;
    Ok(Value::Boolean(regex.is_match(text)))
}

/// Finds all matches in a string
///
/// Args:
///     regex: A regex object created with regex_new
///     text: The string to search for matches
///
/// Returns:
///     An array of match strings
pub fn regex_match_all(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("regex_match_all expects 2 arguments".to_string());
    }

    let regex_obj = &args[0];
    if !is_regex(regex_obj) {
        return Err("First argument must be a regex object created with regex_new".to_string());
    }

    let text = match &args[1] {
        Value::String(s) => s,
        _ => return Err("Second argument must be a string".to_string()),
    };

    let regex = get_compiled_regex(regex_obj)?;

    let mut matches = Vec::new();
    for cap in regex.find_iter(text) {
        matches.push(Value::String(cap.as_str().to_string()));
    }

    Ok(Value::Array(matches))
}

/// Replaces all occurrences of a pattern in a string
///
/// Args:
///     regex: A regex object created with regex_new
///     text: The string to perform replacements on
///     replacement: The replacement string
///
/// Returns:
///     A new string with replacements applied
pub fn regex_replace_all(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("regex_replace_all expects 3 arguments".to_string());
    }

    let regex_obj = &args[0];
    if !is_regex(regex_obj) {
        return Err("First argument must be a regex object created with regex_new".to_string());
    }

    let text = match &args[1] {
        Value::String(s) => s,
        _ => return Err("Second argument must be a string".to_string()),
    };

    let replacement = match &args[2] {
        Value::String(s) => s,
        _ => return Err("Third argument must be a string".to_string()),
    };

    let regex = get_compiled_regex(regex_obj)?;
    let result = regex.replace_all(text, replacement.as_str());

    Ok(Value::String(result.to_string()))
}

/// Splits a string by a regex pattern
///
/// Args:
///     regex: A regex object created with regex_new
///     text: The string to split
///
/// Returns:
///     An array of string parts
pub fn regex_split(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("regex_split expects 2 arguments".to_string());
    }

    let regex_obj = &args[0];
    if !is_regex(regex_obj) {
        return Err("First argument must be a regex object created with regex_new".to_string());
    }

    let text = match &args[1] {
        Value::String(s) => s,
        _ => return Err("Second argument must be a string".to_string()),
    };

    let regex = get_compiled_regex(regex_obj)?;

    let parts: Vec<Value> = regex
        .split(text)
        .map(|s| Value::String(s.to_string()))
        .collect();

    Ok(Value::Array(parts))
}

/// Gets an array of capture groups for the first match
///
/// Args:
///     regex: A regex object created with regex_new
///     text: The string to search for matches
///
/// Returns:
///     An array of captured groups or nil if no match
pub fn regex_capture(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("regex_capture expects 2 arguments".to_string());
    }

    let regex_obj = &args[0];
    if !is_regex(regex_obj) {
        return Err("First argument must be a regex object created with regex_new".to_string());
    }

    let text = match &args[1] {
        Value::String(s) => s,
        _ => return Err("Second argument must be a string".to_string()),
    };

    let regex = get_compiled_regex(regex_obj)?;

    if let Some(caps) = regex.captures(text) {
        let mut groups = Vec::new();

        // First add the full match
        if let Some(m) = caps.get(0) {
            groups.push(Value::String(m.as_str().to_string()));
        } else {
            groups.push(Value::Nil);
        }

        // Then add capture groups (starting from 1)
        for i in 1..caps.len() {
            if let Some(m) = caps.get(i) {
                groups.push(Value::String(m.as_str().to_string()));
            } else {
                groups.push(Value::Nil);
            }
        }

        Ok(Value::Array(groups))
    } else {
        Ok(Value::Nil)
    }
}

/// Checks if a regex pattern syntax is valid
///
/// Args:
///     pattern: A string containing the regex pattern
///
/// Returns:
///     true if the pattern is valid, false otherwise
pub fn regex_is_valid(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("regex_is_valid expects 1 argument".to_string());
    }

    let pattern = match &args[0] {
        Value::String(s) => s,
        _ => return Err("regex_is_valid expects a string pattern".to_string()),
    };

    let processed_pattern = process_pattern(pattern);
    match Regex::new(&processed_pattern) {
        Ok(_) => Ok(Value::Boolean(true)),
        Err(_) => Ok(Value::Boolean(false)),
    }
}

/// Escapes a string so it can be used as a literal in a regex pattern
///
/// Args:
///     text: The string to escape
///
/// Returns:
///     An escaped string safe to use in a regex pattern
pub fn regex_escape(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("regex_escape expects 1 argument".to_string());
    }

    let text = match &args[0] {
        Value::String(s) => s,
        _ => return Err("regex_escape expects a string".to_string()),
    };

    Ok(Value::String(regex::escape(text)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Value;

    // Helper function to create a string Value
    fn string_value(s: &str) -> Value {
        Value::String(s.to_string())
    }

    // Helper function to create a regex object from a pattern
    fn create_regex(pattern: &str) -> Value {
        let args = vec![string_value(pattern)];
        regex_new(args).unwrap()
    }

    #[test]
    fn test_process_pattern() {
        // Test basic character escaping
        assert_eq!(process_pattern("abc"), "abc");

        // Test digit metacharacter
        assert_eq!(process_pattern("\\d+"), "\\d+");

        // Test word metacharacter
        assert_eq!(process_pattern("\\w+"), "\\w+");

        // Test space metacharacter
        assert_eq!(process_pattern("\\s+"), "\\s+");

        // Test backslash escaping
        assert_eq!(process_pattern("\\\\"), "\\\\");

        // Test dot escaping
        assert_eq!(process_pattern("\\."), "\\.");
    }

    #[test]
    fn test_regex_new() {
        // Test with valid pattern
        let args = vec![string_value("[0-9]+")];
        let result = regex_new(args).unwrap();

        if let Value::Array(arr) = result {
            assert_eq!(arr.len(), 2);

            if let Value::String(pattern) = &arr[0] {
                assert_eq!(pattern, "[0-9]+");
            } else {
                panic!("Expected first element to be a string");
            }

            if let Value::String(marker) = &arr[1] {
                assert!(marker.starts_with("__REGEX__"));
            } else {
                panic!("Expected second element to be a string");
            }
        } else {
            panic!("Expected result to be an array");
        }

        // Test with invalid pattern
        let args = vec![string_value("[")];
        assert!(regex_new(args).is_err());

        // Test with wrong argument type
        let args = vec![Value::Number(42.0)];
        assert!(regex_new(args).is_err());

        // Test with wrong number of arguments
        let args = vec![];
        assert!(regex_new(args).is_err());
    }

    #[test]
    fn test_is_regex() {
        // Test with valid regex object
        let regex = create_regex("[0-9]+");
        assert!(is_regex(&regex));

        // Test with invalid values
        assert!(!is_regex(&string_value("not a regex")));
        assert!(!is_regex(&Value::Number(42.0)));
        assert!(!is_regex(&Value::Boolean(true)));
        assert!(!is_regex(&Value::Nil));

        // Test with array that's not a regex
        let mut arr = Vec::new();
        arr.push(string_value("not"));
        arr.push(string_value("a regex"));
        assert!(!is_regex(&Value::Array(arr)));
    }

    #[test]
    fn test_regex_test() {
        // Test with matching pattern
        let regex = create_regex("[0-9]+");
        let args = vec![regex.clone(), string_value("123")];
        let result = regex_test(args).unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Test with non-matching pattern
        let args = vec![regex.clone(), string_value("abc")];
        let result = regex_test(args).unwrap();
        assert_eq!(result, Value::Boolean(false));

        // Test with wrong number of arguments
        let args = vec![regex.clone()];
        assert!(regex_test(args).is_err());

        // Test with wrong first argument type
        let args = vec![string_value("not a regex"), string_value("123")];
        assert!(regex_test(args).is_err());

        // Test with wrong second argument type
        let args = vec![regex.clone(), Value::Number(123.0)];
        assert!(regex_test(args).is_err());
    }

    #[test]
    fn test_regex_match_all() {
        // Test with multiple matches
        let regex = create_regex("[0-9]+");
        let args = vec![regex.clone(), string_value("abc123def456")];
        let result = regex_match_all(args).unwrap();

        if let Value::Array(matches) = result {
            assert_eq!(matches.len(), 2);
            assert_eq!(matches[0], string_value("123"));
            assert_eq!(matches[1], string_value("456"));
        } else {
            panic!("Expected result to be an array");
        }

        // Test with no matches
        let args = vec![regex.clone(), string_value("abcdef")];
        let result = regex_match_all(args).unwrap();

        if let Value::Array(matches) = result {
            assert_eq!(matches.len(), 0);
        } else {
            panic!("Expected result to be an array");
        }
    }

    #[test]
    fn test_regex_replace_all() {
        // Test basic replacement
        let regex = create_regex("[0-9]+");
        let args = vec![regex.clone(), string_value("abc123def456"), string_value("X")];
        let result = regex_replace_all(args).unwrap();
        assert_eq!(result, string_value("abcXdefX"));

        // Test with no matches
        let args = vec![regex.clone(), string_value("abcdef"), string_value("X")];
        let result = regex_replace_all(args).unwrap();
        assert_eq!(result, string_value("abcdef"));

        // Test with capture group references
        let regex = create_regex("([a-z]+)([0-9]+)");
        let args = vec![regex.clone(), string_value("abc123def456"), string_value("$2-$1")];
        let result = regex_replace_all(args).unwrap();
        assert_eq!(result, string_value("123-abc456-def"));
    }

    #[test]
    fn test_regex_split() {
        // Test basic split
        let regex = create_regex(",");
        let args = vec![regex.clone(), string_value("a,b,c")];
        let result = regex_split(args).unwrap();

        if let Value::Array(parts) = result {
            assert_eq!(parts.len(), 3);
            assert_eq!(parts[0], string_value("a"));
            assert_eq!(parts[1], string_value("b"));
            assert_eq!(parts[2], string_value("c"));
        } else {
            panic!("Expected result to be an array");
        }

        // Test split with no matches
        let args = vec![regex.clone(), string_value("abc")];
        let result = regex_split(args).unwrap();

        if let Value::Array(parts) = result {
            assert_eq!(parts.len(), 1);
            assert_eq!(parts[0], string_value("abc"));
        } else {
            panic!("Expected result to be an array");
        }
    }

    #[test]
    fn test_regex_capture() {
        // Test with capture groups
        let regex = create_regex("([a-z]+)=([0-9]+)");
        let args = vec![regex.clone(), string_value("width=100")];
        let result = regex_capture(args).unwrap();

        if let Value::Array(groups) = result {
            assert_eq!(groups.len(), 3);
            assert_eq!(groups[0], string_value("width=100")); // Full match
            assert_eq!(groups[1], string_value("width"));     // First group
            assert_eq!(groups[2], string_value("100"));       // Second group
        } else {
            panic!("Expected result to be an array");
        }

        // Test with no match
        let args = vec![regex.clone(), string_value("no match")];
        let result = regex_capture(args).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_regex_is_valid() {
        // Test with valid pattern
        let args = vec![string_value("[0-9]+")];
        let result = regex_is_valid(args).unwrap();
        assert_eq!(result, Value::Boolean(true));

        // Test with invalid pattern
        let args = vec![string_value("[")];
        let result = regex_is_valid(args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_regex_escape() {
        // Test escaping special characters
        let args = vec![string_value("a.b*c?")];
        let result = regex_escape(args).unwrap();
        assert_eq!(result, string_value("a\\.b\\*c\\?"));
    }

    #[test]
    fn test_pattern_escaping_integration() {
        // This test verifies that the pattern escaping functionality works correctly
        // for common regex metacharacters when used in an end-to-end scenario

        // Test digit pattern (\d+)
        let digit_regex = create_regex("\\d+");
        let args = vec![digit_regex.clone(), string_value("abc123def456")];
        let result = regex_match_all(args).unwrap();

        if let Value::Array(matches) = result {
            assert_eq!(matches.len(), 2);
            assert_eq!(matches[0], string_value("123"));
            assert_eq!(matches[1], string_value("456"));
        } else {
            panic!("Expected result to be an array");
        }

        // Test word pattern (\w+)
        let word_regex = create_regex("\\w+");
        let args = vec![word_regex.clone(), string_value("abc 123 def")];
        let result = regex_match_all(args).unwrap();

        if let Value::Array(matches) = result {
            assert_eq!(matches.len(), 3);
            assert_eq!(matches[0], string_value("abc"));
            assert_eq!(matches[1], string_value("123"));
            assert_eq!(matches[2], string_value("def"));
        } else {
            panic!("Expected result to be an array");
        }

        // Test whitespace pattern (\s+)
        let space_regex = create_regex("\\s+");
        let args = vec![space_regex.clone(), string_value("a b  c")];
        let result = regex_match_all(args).unwrap();

        if let Value::Array(matches) = result {
            assert_eq!(matches.len(), 2);
            assert_eq!(matches[0], string_value(" "));
            assert_eq!(matches[1], string_value("  "));
        } else {
            panic!("Expected result to be an array");
        }

        // Test boundary pattern (\b)
        let boundary_regex = create_regex("\\bword\\b");

        let args = vec![boundary_regex.clone(), string_value("word")];
        let result = regex_test(args).unwrap();
        assert_eq!(result, Value::Boolean(true));

        let args = vec![boundary_regex.clone(), string_value("words")];
        let result = regex_test(args).unwrap();
        assert_eq!(result, Value::Boolean(false));

        // Test multiple metacharacters together
        let complex_regex = create_regex("\\d+\\s+\\w+");
        let args = vec![complex_regex.clone(), string_value("123 abc")];
        let result = regex_test(args).unwrap();
        assert_eq!(result, Value::Boolean(true));

        let args = vec![complex_regex.clone(), string_value("abc 123")];
        let result = regex_test(args).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }
}
