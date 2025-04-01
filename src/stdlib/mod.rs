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

impl StdLib {
    /// Create a new standard library with all available functions
    pub fn new() -> Self {
        let mut stdlib = StdLib {
            functions: HashMap::new(),
        };

        stdlib.register_math_functions();
        stdlib.register_time_functions();
        stdlib.register_string_functions();
        stdlib.register_file_functions();
        stdlib.register_array_functions();
        stdlib.register_json_functions();
        stdlib.register_regex_functions();

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

    /// Register a variadic function (function with variable arguments)
    fn register_variadic(&mut self, name: &str, func: fn(Vec<Value>) -> Result<Value, String>) {
        // For variadic functions, we use arity 0 but the function can accept any number of arguments
        self.register(name, 0, func)
    }

    /// Register all math-related functions
    fn register_math_functions(&mut self) {
        self.register("random", 0, math::random);
        self.register("random_range", 2, math::random_range);
        self.register("abs", 1, math::abs);
        self.register("round", 1, math::round);
        self.register("floor", 1, math::floor);
        self.register("ceil", 1, math::ceil);
        self.register("min", 2, math::min);
        self.register("max", 2, math::max);
    }

    /// Register all time-related functions
    fn register_time_functions(&mut self) {
        self.register("time", 0, time::time);
    }

    /// Register all string-related functions
    fn register_string_functions(&mut self) {
        self.register("len", 1, string::len);
        self.register("upper", 1, string::upper);
        self.register("lower", 1, string::lower);
        self.register("as_string", 1, string::as_string);
    }

    /// Register all file I/O functions
    fn register_file_functions(&mut self) {
        self.register("file", 2, file::file);
    }

    /// Register all array-related functions
    fn register_array_functions(&mut self) {
        self.register_variadic("array", array::array);
        self.register("length", 1, array::length);
        self.register("push", 2, array::push);
        self.register("pop", 1, array::pop);
        self.register("get", 2, array::get);
        self.register("set", 3, array::set);
        self.register("index", 2, array::index);
        self.register("index_set", 3, array::index_set);
        self.register("concat", 2, array::concat);
        self.register("join", 2, array::join);
        self.register_variadic("slice", array::slice);
    }

    /// Register all JSON-related functions
    fn register_json_functions(&mut self) {
        self.register("json_parse", 1, json::json_parse);
        self.register("json_stringify", 1, json::json_stringify);
    }

    /// Register all regex-related functions
    fn register_regex_functions(&mut self) {
        self.register("regex_new", 1, regex::regex_new);
        self.register("regex_test", 2, regex::regex_test);
        self.register("regex_match_all", 2, regex::regex_match_all);
        self.register("regex_replace_all", 3, regex::regex_replace_all);
        self.register("regex_split", 2, regex::regex_split);
        self.register("regex_capture", 2, regex::regex_capture);
        self.register("regex_is_valid", 1, regex::regex_is_valid);
        self.register("regex_escape", 1, regex::regex_escape);
    }
}
