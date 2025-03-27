use crate::interpreter::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get current time in milliseconds since epoch
pub fn time(_args: Vec<Value>) -> Result<Value, String> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let millis = duration.as_millis() as f64;
            Ok(Value::Number(millis))
        },
        Err(_) => Err("time: could not determine current time".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let args = vec![];
        let result = time(args).unwrap();
        if let Value::Number(_) = result {
            // Just check that it returns a number, actual value will vary
            assert!(true);
        } else {
            panic!("Expected number result");
        }
    }
}
