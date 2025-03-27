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

                match (left, &operator.token_type, right) {
                    // Arithmetic operations
                    (Value::Number(a), TokenType::Plus, Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::Number(a), TokenType::Minus, Value::Number(b)) => Ok(Value::Number(a - b)),
                    (Value::Number(a), TokenType::Star, Value::Number(b)) => Ok(Value::Number(a * b)),
                    (Value::Number(a), TokenType::Slash, Value::Number(b)) => {
                        if b == 0.0 {
                            Err(RuntimeError::Error("Division by zero.".to_string()))
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }

                    // String concatenation
                    (Value::String(a), TokenType::Plus, Value::String(b)) => Ok(Value::String(a + &b)),

                    // Comparisons
                    (Value::Number(a), TokenType::Greater, Value::Number(b)) => Ok(Value::Boolean(a > b)),
                    (Value::Number(a), TokenType::GreaterEqual, Value::Number(b)) => Ok(Value::Boolean(a >= b)),
                    (Value::Number(a), TokenType::Less, Value::Number(b)) => Ok(Value::Boolean(a < b)),
                    (Value::Number(a), TokenType::LessEqual, Value::Number(b)) => Ok(Value::Boolean(a <= b)),

                    // Equality
                    (a, TokenType::EqualEqual, b) => Ok(Value::Boolean(self.is_equal(&a, &b))),
                    (a, TokenType::BangEqual, b) => Ok(Value::Boolean(!self.is_equal(&a, &b))),

                    _ => Err(RuntimeError::Error("Invalid binary operation.".to_string())),
                }
            }
            Expr::Variable(name) => {
                match self.environment.get(&name.lexeme) {
                    Some(value) => Ok(value),
                    None => Err(RuntimeError::Error(format!("Undefined variable '{}'.", name.lexeme))),
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
            Expr::Call(callee, _paren, arguments) => {
                let callee_value = self.evaluate(callee)?;

                let mut argument_values = Vec::new();
                for argument in arguments {
                    argument_values.push(self.evaluate(argument)?);
                }

                match callee_value {
                    Value::Function(function) => self.call_function(&function, argument_values),
                    Value::NativeFunction(function) => {
                        // Special case for variadic functions that can take any number of arguments
                        if (function.name == "file" && (argument_values.len() == 2 || argument_values.len() == 3)) ||
                           (function.name == "array") {
                            match (function.function)(argument_values) {
                                Ok(value) => Ok(value),
                                Err(message) => Err(RuntimeError::Error(message)),
                            }
                        } else if argument_values.len() != function.arity {
                            return Err(RuntimeError::Error(format!(
                                "Expected {} arguments but got {}.",
                                function.arity,
                                argument_values.len()
                            )));
                        } else {
                            match (function.function)(argument_values) {
                                Ok(value) => Ok(value),
                                Err(message) => Err(RuntimeError::Error(message)),
                            }
                        }
                    },
                    _ => Err(RuntimeError::Error("Can only call functions.".to_string())),
                }
            }
        }
    }

    fn call_function(&mut self, function: &Rc<Function>, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        if arguments.len() != function.params.len() {
            return Err(RuntimeError::Error(format!(
                "Expected {} arguments but got {}.",
                function.params.len(),
                arguments.len()
            )));
        }

        let previous_env = self.environment.clone();
        self.environment = Environment::new(Some(Box::new(previous_env)));

        for (param, arg) in function.params.iter().zip(arguments.iter()) {
            self.environment.define(param.lexeme.clone(), arg.clone());
        }

        let result = match self.execute_block(&function.body) {
            Ok(_) => Ok(Value::Nil),
            Err(RuntimeError::Return(value)) => Ok(value),
            Err(e) => Err(e),
        };

        self.environment = *self.environment.enclosing.take().unwrap();
        result
    }

    fn execute_block(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
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
        }
    }
}
