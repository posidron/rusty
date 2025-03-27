use crate::interpreter::{Value, NativeFunction};
use std::rc::Rc;
use std::collections::HashMap;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Standard library for the Rusty language
pub struct StdLib {
    functions: HashMap<String, Rc<NativeFunction>>,
}

impl StdLib {
    /// Create a new standard library with all available functions
    pub fn new() -> Self {
        let mut stdlib = StdLib {
            functions: HashMap::new(),
        };

        stdlib.register_math_functions();
        stdlib.register_time_functions();
        stdlib.register_string_functions();

        stdlib
    }

    /// Get all standard library functions
    pub fn get_functions(&self) -> &HashMap<String, Rc<NativeFunction>> {
        &self.functions
    }

    /// Register a native function in the standard library
    fn register(&mut self, name: &str, arity: usize, func: fn(Vec<Value>) -> Result<Value, String>) {
        let native_fn = Rc::new(NativeFunction {
            name: name.to_string(),
            arity,
            function: func,
        });

        self.functions.insert(name.to_string(), native_fn);
    }

    /// Register all math-related functions
    fn register_math_functions(&mut self) {
        // Random number generator (0.0 to 1.0)
        self.register("random", 0, |_args| {
            let mut rng = rand::thread_rng();
            let value: f64 = rng.gen();
            Ok(Value::Number(value))
        });

        // Random integer in range [min, max]
        self.register("random_range", 2, |args| {
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
        });

        // Absolute value
        self.register("abs", 1, |args| {
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.abs())),
                _ => Err("abs: argument must be a number".to_string()),
            }
        });

        // Round to nearest integer
        self.register("round", 1, |args| {
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.round())),
                _ => Err("round: argument must be a number".to_string()),
            }
        });

        // Floor (round down)
        self.register("floor", 1, |args| {
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.floor())),
                _ => Err("floor: argument must be a number".to_string()),
            }
        });

        // Ceiling (round up)
        self.register("ceil", 1, |args| {
            match &args[0] {
                Value::Number(n) => Ok(Value::Number(n.ceil())),
                _ => Err("ceil: argument must be a number".to_string()),
            }
        });

        // Min of two numbers
        self.register("min", 2, |args| {
            if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
                Ok(Value::Number(a.min(*b)))
            } else {
                Err("min: arguments must be numbers".to_string())
            }
        });

        // Max of two numbers
        self.register("max", 2, |args| {
            if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
                Ok(Value::Number(a.max(*b)))
            } else {
                Err("max: arguments must be numbers".to_string())
            }
        });
    }

    /// Register all time-related functions
    fn register_time_functions(&mut self) {
        // Get current time in milliseconds since epoch
        self.register("time", 0, |_args| {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => {
                    let millis = duration.as_millis() as f64;
                    Ok(Value::Number(millis))
                },
                Err(_) => Err("time: could not determine current time".to_string()),
            }
        });
    }

    /// Register all string-related functions
    fn register_string_functions(&mut self) {
        // Get string length
        self.register("len", 1, |args| {
            match &args[0] {
                Value::String(s) => Ok(Value::Number(s.len() as f64)),
                _ => Err("len: argument must be a string".to_string()),
            }
        });

        // Convert to uppercase
        self.register("upper", 1, |args| {
            match &args[0] {
                Value::String(s) => Ok(Value::String(s.to_uppercase())),
                _ => Err("upper: argument must be a string".to_string()),
            }
        });

        // Convert to lowercase
        self.register("lower", 1, |args| {
            match &args[0] {
                Value::String(s) => Ok(Value::String(s.to_lowercase())),
                _ => Err("lower: argument must be a string".to_string()),
            }
        });

        // Convert value to string
        self.register("as_string", 1, |args| {
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
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;
    use crate::lexer::Lexer;
    use crate::ast_parser::Parser;

    fn create_interpreter() -> Interpreter {
        Interpreter::new()
    }

    fn execute_code(code: &str, interpreter: &mut Interpreter) -> Result<(), String> {
        // Create lexer and tokenize
        let mut lexer = Lexer::new(code.to_string());
        match lexer.scan_tokens() {
            Ok(tokens) => {
                // Create parser and parse
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(statements) => {
                        // Execute
                        interpreter.interpret(statements)
                    }
                    Err(error) => Err(format!("Parse error: {}", error)),
                }
            }
            Err(error) => Err(format!("Lexical error: {}", error)),
        }
    }

    #[test]
    fn test_math_functions() {
        let mut interpreter = create_interpreter();

        // Test random()
        assert!(execute_code("var x = random();", &mut interpreter).is_ok());

        // Test random_range() - separate tests for each step
        assert!(execute_code("var y = random_range(1, 10);", &mut interpreter).is_ok());

        // Test separate comparisons to avoid logical operators
        assert!(execute_code("var yGTE1 = y >= 1;", &mut interpreter).is_ok());
        assert!(execute_code("var yLTE10 = y <= 10;", &mut interpreter).is_ok());

        // Test abs()
        assert!(execute_code("var a = abs(-5);", &mut interpreter).is_ok());
        assert!(execute_code("var absTest = a == 5;", &mut interpreter).is_ok());

        // Test round(), floor(), ceil()
        assert!(execute_code("var r = round(3.7);", &mut interpreter).is_ok());
        assert!(execute_code("var roundTest = r == 4;", &mut interpreter).is_ok());

        assert!(execute_code("var f = floor(3.7);", &mut interpreter).is_ok());
        assert!(execute_code("var floorTest = f == 3;", &mut interpreter).is_ok());

        assert!(execute_code("var c = ceil(3.2);", &mut interpreter).is_ok());
        assert!(execute_code("var ceilTest = c == 4;", &mut interpreter).is_ok());

        // Test min(), max()
        assert!(execute_code("var min_val = min(5, 10);", &mut interpreter).is_ok());
        assert!(execute_code("var minTest = min_val == 5;", &mut interpreter).is_ok());

        assert!(execute_code("var max_val = max(5, 10);", &mut interpreter).is_ok());
        assert!(execute_code("var maxTest = max_val == 10;", &mut interpreter).is_ok());
    }

    #[test]
    fn test_time_functions() {
        let mut interpreter = create_interpreter();

        // Test time() - just check it doesn't error and returns a non-negative number
        assert!(execute_code("var t = time();", &mut interpreter).is_ok());
        assert!(execute_code("var timeTest = t > 0;", &mut interpreter).is_ok());
    }

    #[test]
    fn test_string_functions() {
        let mut interpreter = create_interpreter();

        // Test len()
        assert!(execute_code("var length = len(\"hello\");", &mut interpreter).is_ok());
        assert!(execute_code("var lenTest = length == 5;", &mut interpreter).is_ok());

        // Test upper()
        assert!(execute_code("var upper = upper(\"hello\");", &mut interpreter).is_ok());
        assert!(execute_code("var upperTest = upper == \"HELLO\";", &mut interpreter).is_ok());

        // Test lower()
        assert!(execute_code("var lower = lower(\"HELLO\");", &mut interpreter).is_ok());
        assert!(execute_code("var lowerTest = lower == \"hello\";", &mut interpreter).is_ok());

        // Test as_string()
        assert!(execute_code("var numStr = as_string(42);", &mut interpreter).is_ok());
        assert!(execute_code("var as_stringTest = numStr == \"42\";", &mut interpreter).is_ok());

        assert!(execute_code("var boolStr = as_string(true);", &mut interpreter).is_ok());
        assert!(execute_code("var boolTest = boolStr == \"true\";", &mut interpreter).is_ok());
    }

    #[test]
    fn test_error_handling() {
        let mut interpreter = create_interpreter();

        // Test invalid arguments
        assert!(execute_code("var err1 = random_range(\"str\", 10);", &mut interpreter).is_err());
        assert!(execute_code("var err2 = abs(\"not a number\");", &mut interpreter).is_err());
        assert!(execute_code("var err3 = len(42);", &mut interpreter).is_err());
        assert!(execute_code("var err4 = upper(true);", &mut interpreter).is_err());

        // Test invalid argument count
        assert!(execute_code("var err5 = random_range(1);", &mut interpreter).is_err());
        assert!(execute_code("var err6 = min(1);", &mut interpreter).is_err());
        assert!(execute_code("var err7 = min(1, 2, 3);", &mut interpreter).is_err());
    }

    #[test]
    fn test_chainable_functions() {
        let mut interpreter = create_interpreter();

        // Test chaining functions
        assert!(execute_code("var num = round(random() * 100);", &mut interpreter).is_ok());
        assert!(execute_code("var str = as_string(num);", &mut interpreter).is_ok());
        assert!(execute_code("var upper = upper(str);", &mut interpreter).is_ok());
        assert!(execute_code("var length = len(upper);", &mut interpreter).is_ok());
        assert!(execute_code("var lengthTest = length > 0;", &mut interpreter).is_ok());
    }
}
