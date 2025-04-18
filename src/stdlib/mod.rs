pub mod math;
pub mod time;
pub mod string;
pub mod file;
pub mod array;
pub mod json;
pub mod regex;

use crate::interpreter::{Value, NativeFunction};
use std::rc::Rc;
use std::collections::HashMap;

/// Standard library for the Rusty language
pub struct StdLib {
    functions: HashMap<String, Rc<NativeFunction>>,
}

// Global namespace constants - accessible from the getter functions
static mut MATH_NAMESPACE: Option<Value> = None;
static mut STRING_NAMESPACE: Option<Value> = None;
static mut FILE_NAMESPACE: Option<Value> = None;
static mut TIME_NAMESPACE: Option<Value> = None;
static mut JSON_NAMESPACE: Option<Value> = None;
static mut REGEX_NAMESPACE: Option<Value> = None;

// Namespace accessor functions
fn get_math_namespace(_args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        if let Some(ref ns) = MATH_NAMESPACE {
            Ok(ns.clone())
        } else {
            Err("Math namespace not initialized".to_string())
        }
    }
}

fn get_array_namespace(args: Vec<Value>) -> Result<Value, String> {
    // Always create and return an array with the given arguments
    return array::array(args);
}

fn get_string_namespace(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        // If called with no arguments, return the namespace
        unsafe {
            if let Some(ref ns) = STRING_NAMESPACE {
                return Ok(ns.clone());
            } else {
                return Err("String namespace not initialized".to_string());
            }
        }
    } else {
        // If called with arguments, convert to string
        return string::as_string(vec![args[0].clone()]);
    }
}

fn get_file_namespace(_args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        if let Some(ref ns) = FILE_NAMESPACE {
            Ok(ns.clone())
        } else {
            Err("File namespace not initialized".to_string())
        }
    }
}

fn get_time_namespace(_args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        if let Some(ref ns) = TIME_NAMESPACE {
            Ok(ns.clone())
        } else {
            Err("Time namespace not initialized".to_string())
        }
    }
}

fn get_json_namespace(_args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        if let Some(ref ns) = JSON_NAMESPACE {
            Ok(ns.clone())
        } else {
            Err("JSON namespace not initialized".to_string())
        }
    }
}

fn get_regex_namespace(_args: Vec<Value>) -> Result<Value, String> {
    unsafe {
        if let Some(ref ns) = REGEX_NAMESPACE {
            Ok(ns.clone())
        } else {
            Err("Regex namespace not initialized".to_string())
        }
    }
}

impl StdLib {
    /// Create a new standard library with all available functions
    pub fn new() -> Self {
        let mut stdlib = StdLib {
            functions: HashMap::new(),
        };

        // Register namespaces for OOP style
        stdlib.register_all_namespaces();

        stdlib
    }

    /// Get all standard library functions
    pub fn get_functions(&self) -> &HashMap<String, Rc<NativeFunction>> {
        &self.functions
    }

    /// Register all namespaces
    fn register_all_namespaces(&mut self) {
        // Register Math namespace
        self.register_math_namespace();

        // Register Array namespace
        self.register_array_namespace();

        // Register String namespace
        self.register_string_namespace();

        // Register File namespace
        self.register_file_namespace();

        // Register Time namespace
        self.register_time_namespace();

        // Register JSON namespace
        self.register_json_namespace();

        // Register Regex namespace
        self.register_regex_namespace();
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

    /// Register a Math namespace with math constants and functions
    fn register_math_namespace(&mut self) {
        let mut math_namespace = Value::new_namespace("Math");

        // Add properties (PI, E)
        if let Value::Namespace(_, props) = &mut math_namespace {
            props.insert("PI".to_string(), Value::Number(std::f64::consts::PI));
            props.insert("E".to_string(), Value::Number(std::f64::consts::E));
        }

        // Create Math namespace functions
        for (name, arity, func) in math::get_functions() {
            let math_fn = Rc::new(NativeFunction {
                name: format!("Math.{}", name),
                arity: arity,
                function: func,
            });

            if let Value::Namespace(_, props) = &mut math_namespace {
                props.insert(name.to_string(), Value::NativeFunction(math_fn.clone()));
            }

            // Also register the function directly for static access
            self.register(&format!("Math.{}", name), arity, func);
        }

        // Add PI and E as static properties too
        self.register("Math.PI", 0, |_| Ok(Value::Number(std::f64::consts::PI)));
        self.register("Math.E", 0, |_| Ok(Value::Number(std::f64::consts::E)));

        // Store the namespace in the global variable
        unsafe {
            MATH_NAMESPACE = Some(math_namespace);
        }

        // Register the Math namespace accessor function
        self.register("Math", 0, get_math_namespace);
    }

    /// Create an Array namespace with all array functions
    fn register_array_namespace(&mut self) {
        // Register the Array constructor function (creates arrays)
        self.register("Array", 0, get_array_namespace);

        // Register all array methods as static methods on Array namespace
        self.register("Array.create", 0, array::array);
        self.register("Array.length", 1, array::length);
        self.register("Array.push", 2, array::push);
        self.register("Array.pop", 1, array::pop);
        self.register("Array.get", 2, array::get);
        self.register("Array.set", 3, array::set);
        self.register("Array.concat", 2, array::concat);
        self.register("Array.join", 2, array::join);
    }

    /// Create a String namespace with string utility functions
    fn register_string_namespace(&mut self) {
        let mut string_namespace = Value::new_namespace("String");

        // Create String namespace functions
        let len_fn = Rc::new(NativeFunction {
            name: "String.length".to_string(),
            arity: 1,
            function: string::len,
        });

        let upper_fn = Rc::new(NativeFunction {
            name: "String.upper".to_string(),
            arity: 1,
            function: string::upper,
        });

        let lower_fn = Rc::new(NativeFunction {
            name: "String.lower".to_string(),
            arity: 1,
            function: string::lower,
        });

        let as_string_fn = Rc::new(NativeFunction {
            name: "String.string".to_string(),
            arity: 1,
            function: string::as_string,
        });

        // Add methods to String namespace
        if let Value::Namespace(_, props) = &mut string_namespace {
            props.insert("length".to_string(), Value::NativeFunction(len_fn.clone()));
            props.insert("upper".to_string(), Value::NativeFunction(upper_fn.clone()));
            props.insert("lower".to_string(), Value::NativeFunction(lower_fn.clone()));
            props.insert("string".to_string(), Value::NativeFunction(as_string_fn.clone()));
        }

        // Store the namespace in the global variable
        unsafe {
            STRING_NAMESPACE = Some(string_namespace);
        }

        // Register the String namespace accessor function
        self.register("String", 0, get_string_namespace);

        // Also register static methods on String namespace
        self.register("String.length", 1, string::len);
        self.register("String.upper", 1, string::upper);
        self.register("String.lower", 1, string::lower);
        self.register("String.string", 1, string::as_string);
    }

    /// Register a file namespace with file I/O operations
    fn register_file_namespace(&mut self) {
        let mut file_namespace = Value::new_namespace("File");

        // Create File namespace functions
        let read_fn = Rc::new(NativeFunction {
            name: "File.read".to_string(),
            arity: 1,
            function: file::read,
        });

        let write_fn = Rc::new(NativeFunction {
            name: "File.write".to_string(),
            arity: 2,
            function: file::write,
        });

        let append_fn = Rc::new(NativeFunction {
            name: "File.append".to_string(),
            arity: 2,
            function: file::append,
        });

        let exists_fn = Rc::new(NativeFunction {
            name: "File.exists".to_string(),
            arity: 1,
            function: file::exists,
        });

        let delete_fn = Rc::new(NativeFunction {
            name: "File.delete".to_string(),
            arity: 1,
            function: file::delete,
        });

        // Add methods to File namespace
        if let Value::Namespace(_, props) = &mut file_namespace {
            props.insert("read".to_string(), Value::NativeFunction(read_fn));
            props.insert("write".to_string(), Value::NativeFunction(write_fn));
            props.insert("append".to_string(), Value::NativeFunction(append_fn));
            props.insert("exists".to_string(), Value::NativeFunction(exists_fn));
            props.insert("delete".to_string(), Value::NativeFunction(delete_fn));
        }

        // Store the namespace in the global variable
        unsafe {
            FILE_NAMESPACE = Some(file_namespace);
        }

        // Register the File namespace accessor function
        self.register("File", 0, get_file_namespace);
    }

    /// Register a Time namespace
    fn register_time_namespace(&mut self) {
        let mut time_namespace = Value::new_namespace("Time");

        // Create Time namespace functions
        let now_fn = Rc::new(NativeFunction {
            name: "Time.now".to_string(),
            arity: 0,
            function: time::time,
        });

        // Add methods to Time namespace
        if let Value::Namespace(_, props) = &mut time_namespace {
            props.insert("now".to_string(), Value::NativeFunction(now_fn));
        }

        // Store the namespace in the global variable
        unsafe {
            TIME_NAMESPACE = Some(time_namespace);
        }

        // Register the Time namespace accessor function
        self.register("Time", 0, get_time_namespace);
    }

    /// Register a JSON namespace
    fn register_json_namespace(&mut self) {
        let mut json_namespace = Value::new_namespace("JSON");

        // Create JSON namespace functions
        let parse_fn = Rc::new(NativeFunction {
            name: "JSON.parse".to_string(),
            arity: 1,
            function: json::json_parse,
        });

        let stringify_fn = Rc::new(NativeFunction {
            name: "JSON.stringify".to_string(),
            arity: 1,
            function: json::json_stringify,
        });

        // Add methods to JSON namespace
        if let Value::Namespace(_, props) = &mut json_namespace {
            props.insert("parse".to_string(), Value::NativeFunction(parse_fn));
            props.insert("stringify".to_string(), Value::NativeFunction(stringify_fn));
        }

        // Store the namespace in the global variable
        unsafe {
            JSON_NAMESPACE = Some(json_namespace);
        }

        // Register the JSON namespace accessor function
        self.register("JSON", 0, get_json_namespace);
    }

    /// Register a Regex namespace
    fn register_regex_namespace(&mut self) {
        let mut regex_namespace = Value::new_namespace("Regex");

        // Create Regex namespace functions
        let new_fn = Rc::new(NativeFunction {
            name: "Regex.new".to_string(),
            arity: 1,
            function: regex::regex_new,
        });

        let test_fn = Rc::new(NativeFunction {
            name: "Regex.test".to_string(),
            arity: 2,
            function: regex::regex_test,
        });

        let match_fn = Rc::new(NativeFunction {
            name: "Regex.match".to_string(),
            arity: 2,
            function: regex::regex_match_all,
        });

        let replace_fn = Rc::new(NativeFunction {
            name: "Regex.replace".to_string(),
            arity: 3,
            function: regex::regex_replace_all,
        });

        let split_fn = Rc::new(NativeFunction {
            name: "Regex.split".to_string(),
            arity: 2,
            function: regex::regex_split,
        });

        let capture_fn = Rc::new(NativeFunction {
            name: "Regex.capture".to_string(),
            arity: 2,
            function: regex::regex_capture,
        });

        let is_valid_fn = Rc::new(NativeFunction {
            name: "Regex.is_valid".to_string(),
            arity: 1,
            function: regex::regex_is_valid,
        });

        let escape_fn = Rc::new(NativeFunction {
            name: "Regex.escape".to_string(),
            arity: 1,
            function: regex::regex_escape,
        });

        // Add methods to Regex namespace
        if let Value::Namespace(_, props) = &mut regex_namespace {
            props.insert("new".to_string(), Value::NativeFunction(new_fn));
            props.insert("test".to_string(), Value::NativeFunction(test_fn));
            props.insert("match".to_string(), Value::NativeFunction(match_fn));
            props.insert("replace".to_string(), Value::NativeFunction(replace_fn));
            props.insert("split".to_string(), Value::NativeFunction(split_fn));
            props.insert("capture".to_string(), Value::NativeFunction(capture_fn));
            props.insert("is_valid".to_string(), Value::NativeFunction(is_valid_fn));
            props.insert("escape".to_string(), Value::NativeFunction(escape_fn));
        }

        // Store the namespace in the global variable
        unsafe {
            REGEX_NAMESPACE = Some(regex_namespace);
        }

        // Register the Regex namespace accessor function
        self.register("Regex", 0, get_regex_namespace);
    }
}
