use crate::interpreter::Value;
use rand::Rng;

/// Returns a tuple of (name, arity, function) for all math functions
pub fn get_functions() -> Vec<(&'static str, usize, fn(Vec<Value>) -> Result<Value, String>)> {
    vec![
        ("random", 0, random),
        ("random_range", 2, random_range),
        ("abs", 1, abs),
        ("round", 1, round),
        ("floor", 1, floor),
        ("ceil", 1, ceil),
        ("min", 2, min),
        ("max", 2, max),
    ]
}

/// Random number generator (0.0 to 1.0)
pub fn random(_args: Vec<Value>) -> Result<Value, String> {
    let mut rng = rand::thread_rng();
    let value: f64 = rng.gen();
    Ok(Value::Number(value))
}

/// Random integer in range [min, max]
pub fn random_range(args: Vec<Value>) -> Result<Value, String> {
    if let (Value::Number(min), Value::Number(max)) = (&args[0], &args[1]) {
        let min = min.floor() as i64;
        let max = max.floor() as i64;

        if min > max {
            return Err(format!("random_range: min ({}) must be less than or equal to max ({})", min, max));
        }

        let mut rng = rand::thread_rng();
        let value = rng.gen_range(min..=max) as f64;
        Ok(Value::Number(value))
    } else {
        Err("random_range: arguments must be numbers".to_string())
    }
}

/// Absolute value
pub fn abs(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err("abs: argument must be a number".to_string()),
    }
}

/// Round to nearest integer
pub fn round(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.round())),
        _ => Err("round: argument must be a number".to_string()),
    }
}

/// Floor (round down)
pub fn floor(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err("floor: argument must be a number".to_string()),
    }
}

/// Ceiling (round up)
pub fn ceil(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err("ceil: argument must be a number".to_string()),
    }
}

/// Min of two numbers
pub fn min(args: Vec<Value>) -> Result<Value, String> {
    if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
        Ok(Value::Number(a.min(*b)))
    } else {
        Err("min: arguments must be numbers".to_string())
    }
}

/// Max of two numbers
pub fn max(args: Vec<Value>) -> Result<Value, String> {
    if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
        Ok(Value::Number(a.max(*b)))
    } else {
        Err("max: arguments must be numbers".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        let args = vec![Value::Number(-5.0)];
        let result = abs(args).unwrap();
        if let Value::Number(n) = result {
            assert_eq!(n, 5.0);
        } else {
            panic!("Expected number result");
        }
    }

    #[test]
    fn test_min_max() {
        let args = vec![Value::Number(5.0), Value::Number(10.0)];

        let min_result = min(args.clone()).unwrap();
        if let Value::Number(n) = min_result {
            assert_eq!(n, 5.0);
        } else {
            panic!("Expected number result");
        }

        let max_result = max(args).unwrap();
        if let Value::Number(n) = max_result {
            assert_eq!(n, 10.0);
        } else {
            panic!("Expected number result");
        }
    }
}
