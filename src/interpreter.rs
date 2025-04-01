use crate::ast_parser::{Expr, Literal, Stmt};
use crate::lexer::{Token, TokenType};
use crate::stdlib::StdLib;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Function(Rc<Function>),
    NativeFunction(Rc<NativeFunction>),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Namespace(String, HashMap<String, Value>),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub function: fn(Vec<Value>) -> Result<Value, String>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        // Two functions are equal if they have the same name
        // This is a simplification, but works for our purposes
        self.name.lexeme == other.name.lexeme
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        // Two native functions are equal if they have the same name
        self.name == other.name
    }
}

impl Function {
    pub fn new(name: String, params: Vec<Token>, body: Vec<Stmt>) -> Rc<Self> {
        Rc::new(Function {
            name: Token::new(TokenType::Identifier(name.clone()), name, None, 0, 0),
            params,
            body,
        })
    }
}

impl NativeFunction {
    pub fn new(name: String, arity: usize, function: fn(Vec<Value>) -> Result<Value, String>) -> Rc<Self> {
        Rc::new(NativeFunction {
            name,
            arity,
            function,
        })
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    Return(Value),
    Error(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::Return(_) => write!(f, "Return value outside of function"),
            RuntimeError::Error(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<String> for RuntimeError {
    fn from(error: String) -> Self {
        RuntimeError::Error(error)
    }
}

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            true
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            false
        }
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            environment: Environment::new(None),
        };

        // Initialize standard library
        let stdlib = StdLib::new();
        for (name, function) in stdlib.get_functions() {
            interpreter.environment.define(name.clone(), Value::NativeFunction(function.clone()));
        }

        interpreter
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => {},
                Err(RuntimeError::Return(_)) => return Err("Return statement outside of function".to_string()),
                Err(RuntimeError::Error(msg)) => return Err(msg),
            }
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", self.stringify(value));
                Ok(())
            }
            Stmt::Var(name, initializer) => {
                let value = if let Some(init) = initializer {
                    self.evaluate(init)?
                } else {
                    Value::Nil
                };
                self.environment.define(name.lexeme.clone(), value);
                Ok(())
            }
            Stmt::Block(statements) => {
                let previous = self.environment.clone();
                self.environment = Environment::new(Some(Box::new(previous)));

                for stmt in statements {
                    match self.execute(stmt) {
                        Ok(_) => {},
                        Err(e) => {
                            self.environment = *self.environment.enclosing.take().unwrap();
                            return Err(e);
                        }
                    }
                }

                self.environment = *self.environment.enclosing.take().unwrap();
                Ok(())
            }
            Stmt::If(condition, then_branch, else_branch) => {
                let condition_value = self.evaluate(condition)?;
                if self.is_truthy(&condition_value) {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
                Ok(())
            }
            Stmt::While(condition, body) => {
                loop {
                    let condition_value = self.evaluate(condition)?;
                    if !self.is_truthy(&condition_value) {
                        break;
                    }
                    match self.execute(body) {
                        Ok(_) => {},
                        Err(RuntimeError::Return(value)) => return Err(RuntimeError::Return(value)),
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            Stmt::Function(name, params, body) => {
                let function = Function::new(name.lexeme.clone(), params.clone(), body.clone());
                self.environment.define(name.lexeme.clone(), Value::Function(function));
                Ok(())
            }
            Stmt::Return(_keyword, value) => {
                let return_value = if let Some(value) = value {
                    self.evaluate(value)?
                } else {
                    Value::Nil
                };
                Err(RuntimeError::Return(return_value))
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(literal) => Ok(match literal {
                Literal::Number(n) => Value::Number(*n),
                Literal::String(s) => Value::String(s.clone()),
                Literal::Boolean(b) => Value::Boolean(*b),
                Literal::Nil => Value::Nil,
            }),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary(operator, right) => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        if let Value::Number(n) = right {
                            Ok(Value::Number(-n))
                        } else {
                            Err(RuntimeError::Error("Operand must be a number.".to_string()))
                        }
                    }
                    TokenType::Bang => Ok(Value::Boolean(!self.is_truthy(&right))),
                    _ => Err(RuntimeError::Error("Invalid unary operator.".to_string())),
                }
            }
            Expr::Binary(left, operator, right) => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Minus => {
                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                            _ => Err(RuntimeError::Error("Operands must be numbers.".to_string())),
                        }
                    }
                    TokenType::Slash => {
                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => {
                                if b == 0.0 {
                                    Err(RuntimeError::Error("Division by zero.".to_string()))
                                } else {
                                    Ok(Value::Number(a / b))
                                }
                            }
                            _ => Err(RuntimeError::Error("Operands must be numbers.".to_string())),
                        }
                    }
                    TokenType::Star => {
                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                            _ => Err(RuntimeError::Error("Operands must be numbers.".to_string())),
                        }
                    }
                    TokenType::Plus => {
                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                            (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
                            _ => Err(RuntimeError::Error("Operands must be two numbers or two strings.".to_string())),
                        }
                    }
                    TokenType::Greater => {
                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
                            _ => Err(RuntimeError::Error("Operands must be numbers.".to_string())),
                        }
                    }
                    TokenType::GreaterEqual => {
                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
                            _ => Err(RuntimeError::Error("Operands must be numbers.".to_string())),
                        }
                    }
                    TokenType::Less => {
                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
                            _ => Err(RuntimeError::Error("Operands must be numbers.".to_string())),
                        }
                    }
                    TokenType::LessEqual => {
                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
                            _ => Err(RuntimeError::Error("Operands must be numbers.".to_string())),
                        }
                    }
                    TokenType::BangEqual => Ok(Value::Boolean(left != right)),
                    TokenType::EqualEqual => Ok(Value::Boolean(left == right)),
                    _ => Err(RuntimeError::Error("Invalid binary operator.".to_string())),
                }
            }
            Expr::Variable(name) => {
                if let Some(value) = self.environment.get(&name.lexeme) {
                    Ok(value)
                } else {
                    Err(RuntimeError::Error(format!("Undefined variable '{}'.", name.lexeme)))
                }
            }
            Expr::Assign(name, value) => {
                let value = self.evaluate(value)?;
                if self.environment.assign(&name.lexeme, value.clone()) {
                    Ok(value)
                } else {
                    Err(RuntimeError::Error(format!("Undefined variable '{}'.", name.lexeme)))
                }
            }
            Expr::Logical(left, operator, right) => {
                let left = self.evaluate(left)?;

                match operator.token_type {
                    TokenType::Or => {
                        if self.is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                    TokenType::And => {
                        if !self.is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                    _ => return Err(RuntimeError::Error("Invalid logical operator.".to_string())),
                }

                self.evaluate(right)
            }
            Expr::Call(callee, _paren, arguments) => {
                let callee_value = self.evaluate(callee)?;
                let mut args = Vec::new();
                for argument in arguments {
                    args.push(self.evaluate(argument)?);
                }

                if !callee_value.is_callable() {
                    return Err(RuntimeError::Error(format!("Can only call functions and classes.")));
                }

                if let Value::Function(function) = &callee_value {
                    if args.len() != function.params.len() {
                        return Err(RuntimeError::Error(format!(
                            "Expected {} arguments but got {}.",
                            function.params.len(),
                            args.len()
                        )));
                    }

                    // Create a new environment for the function call
                    let previous = self.environment.clone();
                    self.environment = Environment::new(Some(Box::new(previous)));

                    // Bind arguments to parameters
                    for (param, arg) in function.params.iter().zip(args.iter()) {
                        self.environment.define(param.lexeme.clone(), arg.clone());
                    }

                    // Execute function body
                    let mut return_value = Value::Nil;
                    for stmt in &function.body {
                        match self.execute(stmt) {
                            Ok(_) => {},
                            Err(RuntimeError::Return(value)) => {
                                return_value = value;
                                break;
                            },
                            Err(e) => {
                                // Restore environment and propagate error
                                self.environment = *self.environment.enclosing.take().unwrap();
                                return Err(e);
                            }
                        }
                    }

                    // Restore environment
                    self.environment = *self.environment.enclosing.take().unwrap();
                    Ok(return_value)
                } else if let Value::NativeFunction(function) = &callee_value {
                    if function.arity != 0 && args.len() != function.arity {
                        return Err(RuntimeError::Error(format!(
                            "Expected {} arguments but got {}.",
                            function.arity,
                            args.len()
                        )));
                    }

                    match (function.function)(args) {
                        Ok(value) => Ok(value),
                        Err(message) => Err(RuntimeError::Error(message)),
                    }
                } else {
                    Err(RuntimeError::Error("Can only call functions and classes.".to_string()))
                }
            },
            Expr::Get(object, name) => {
                let object_value = self.evaluate(object)?;

                // Handle property access for different types
                match &object_value {
                    Value::Object(_) | Value::Namespace(_, _) => {
                        if let Some(property) = object_value.get_property(&name.lexeme) {
                            Ok(property)
                        } else {
                            Err(RuntimeError::Error(format!("Property '{}' not found.", name.lexeme)))
                        }
                    },
                    Value::Array(elements) => {
                        match name.lexeme.as_str() {
                            "length" => Ok(Value::Number(elements.len() as f64)),
                            _ => Err(RuntimeError::Error(format!("Array has no property '{}'.", name.lexeme)))
                        }
                    },
                    Value::String(s) => {
                        match name.lexeme.as_str() {
                            "length" => Ok(Value::Number(s.len() as f64)),
                            _ => Err(RuntimeError::Error(format!("String has no property '{}'.", name.lexeme)))
                        }
                    },
                    _ => Err(RuntimeError::Error(format!("Cannot access properties of non-object value.")))
                }
            },
            Expr::Method(object, name, arguments) => {
                let object_value = self.evaluate(object)?;
                let mut args = Vec::new();
                for argument in arguments {
                    args.push(self.evaluate(argument)?);
                }

                // Handle method calls for different types
                match &object_value {
                    Value::Object(_) | Value::Namespace(_, _) => {
                        let name_str = &name.lexeme;
                        if let Some(method) = object_value.get_property(name_str) {
                            if method.is_callable() {
                                if let Value::NativeFunction(function) = &method {
                                    if function.arity != 0 && args.len() != function.arity {
                                        return Err(RuntimeError::Error(format!(
                                            "Expected {} arguments but got {}.",
                                            function.arity,
                                            args.len()
                                        )));
                                    }

                                    match (function.function)(args) {
                                        Ok(value) => Ok(value),
                                        Err(message) => Err(RuntimeError::Error(message)),
                                    }
                                } else if let Value::Function(function) = &method {
                                    // Similar implementation as for regular function calls
                                    if args.len() != function.params.len() {
                                        return Err(RuntimeError::Error(format!(
                                            "Expected {} arguments but got {}.",
                                            function.params.len(),
                                            args.len()
                                        )));
                                    }

                                    // Create a new environment for the function call
                                    let previous = self.environment.clone();
                                    self.environment = Environment::new(Some(Box::new(previous)));

                                    // Bind arguments to parameters
                                    for (param, arg) in function.params.iter().zip(args.iter()) {
                                        self.environment.define(param.lexeme.clone(), arg.clone());
                                    }

                                    // Execute function body
                                    let mut return_value = Value::Nil;
                                    for stmt in &function.body {
                                        match self.execute(stmt) {
                                            Ok(_) => {},
                                            Err(RuntimeError::Return(value)) => {
                                                return_value = value;
                                                break;
                                            },
                                            Err(e) => {
                                                // Restore environment and propagate error
                                                self.environment = *self.environment.enclosing.take().unwrap();
                                                return Err(e);
                                            }
                                        }
                                    }

                                    // Restore environment
                                    self.environment = *self.environment.enclosing.take().unwrap();
                                    Ok(return_value)
                                } else {
                                    Err(RuntimeError::Error("Method is not callable.".to_string()))
                                }
                            } else {
                                Err(RuntimeError::Error(format!("Property '{}' is not a method.", name.lexeme)))
                            }
                        } else {
                            Err(RuntimeError::Error(format!("Method '{}' not found.", name.lexeme)))
                        }
                    },
                    _ => Err(RuntimeError::Error(format!("Cannot call methods on non-object value.")))
                }
            },
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate(element)?);
                }
                Ok(Value::Array(values))
            },
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Nil, Value::Nil) => true,
            (Value::Nil, _) => false,
            (_, Value::Nil) => false,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }

    fn stringify(&self, value: Value) -> String {
        match value {
            Value::Number(n) => {
                let text = n.to_string();
                if text.ends_with(".0") {
                    text[..text.len() - 2].to_string()
                } else {
                    text
                }
            },
            Value::String(s) => s,
            Value::Boolean(b) => b.to_string(),
            Value::Nil => "nil".to_string(),
            Value::Function(f) => format!("<fn {}>", f.name.lexeme),
            Value::NativeFunction(f) => format!("<native fn {}>", f.name),
            Value::Array(elements) => {
                let mut result = String::from("[");
                for (i, value) in elements.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&self.stringify(value.clone()));
                }
                result.push_str("]");
                result
            },
            Value::Object(properties) => {
                let mut result = String::from("{");
                for (i, (key, value)) in properties.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&format!("{}: {}", key, self.stringify(value.clone())));
                }
                result.push_str("}");
                result
            },
            Value::Namespace(ns_name, properties) => {
                let mut result = String::new();
                result.push_str("[Namespace: ");
                result.push_str(&ns_name);
                result.push_str(" {");

                for (i, (key, _value)) in properties.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(key);
                }

                result.push_str("}]");
                result
            },
        }
    }
}

impl Value {
    // Helper method to check if a value is a callable function
    pub fn is_callable(&self) -> bool {
        matches!(self, Value::Function(_) | Value::NativeFunction(_))
    }

    // Helper method to get a property from an object or namespace
    pub fn get_property(&self, name: &str) -> Option<Value> {
        match self {
            Value::Object(properties) | Value::Namespace(_, properties) => properties.get(name).cloned(),
            _ => None,
        }
    }

    // Helper method to set a property on an object
    pub fn set_property(&mut self, name: &str, value: Value) -> Result<(), String> {
        match self {
            Value::Object(properties) => {
                properties.insert(name.to_string(), value);
                Ok(())
            },
            Value::Namespace(_, properties) => {
                properties.insert(name.to_string(), value);
                Ok(())
            },
            _ => Err(format!("Cannot set property '{}' on non-object value", name)),
        }
    }

    // Create a new empty object
    pub fn new_object() -> Self {
        Value::Object(HashMap::new())
    }

    // Create a new namespace with given name
    pub fn new_namespace(name: &str) -> Self {
        Value::Namespace(name.to_string(), HashMap::new())
    }
}
