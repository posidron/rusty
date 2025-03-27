use rustjs::{Lexer, Parser, Interpreter};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();

    if args.len() > 1 {
        // File mode: execute script from file
        let file_path = &args[1];
        if !Path::new(file_path).exists() {
            eprintln!("Error: File '{}' not found", file_path);
            std::process::exit(1);
        }

        match fs::read_to_string(file_path) {
            Ok(source) => {
                println!("Executing script from file: {}", file_path);
                execute_source(&source, &mut interpreter);
            }
            Err(error) => {
                eprintln!("Error reading file '{}': {}", file_path, error);
                std::process::exit(1);
            }
        }
    } else {
        // Interactive REPL mode
        repl_mode(&mut interpreter);
    }
}

fn execute_source(source: &str, interpreter: &mut Interpreter) -> bool {
    // Create lexer and tokenize
    let mut lexer = Lexer::new(source.to_string());
    match lexer.scan_tokens() {
        Ok(tokens) => {
            // Create parser and parse
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(statements) => {
                    // Execute
                    match interpreter.interpret(statements) {
                        Ok(_) => true,
                        Err(error) => {
                            println!("Runtime error: {}", error);
                            false
                        }
                    }
                }
                Err(error) => {
                    println!("Parse error: {}", error);
                    println!("ERROR OCCURRED AT TOKEN NUMBER: {}", parser.current);
                    false
                }
            }
        }
        Err(error) => {
            println!("Lexical error: {}", error);
            false
        }
    }
}

fn repl_mode(interpreter: &mut Interpreter) {
    println!("Rusty Language REPL");
    println!("Type '.exit' to quit, '.help' for help");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Handle special commands
        match input {
            ".exit" => break,
            ".help" => {
                println!("Available commands:");
                println!(".exit - Exit the REPL");
                println!(".help - Show this help message");
                println!(".clear - Clear the screen");
                println!(".load <filename> - Load and execute a script file");
                continue;
            }
            ".clear" => {
                // Clear screen (works on Unix-like systems)
                print!("\x1B[2J\x1B[1;1H");
                continue;
            }
            _ if input.starts_with(".load ") => {
                let parts: Vec<&str> = input.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    let file_path = parts[1];
                    if !Path::new(file_path).exists() {
                        println!("Error: File '{}' not found", file_path);
                        continue;
                    }

                    match fs::read_to_string(file_path) {
                        Ok(source) => {
                            println!("Executing script from file: {}", file_path);
                            execute_source(&source, interpreter);
                        }
                        Err(error) => {
                            println!("Error reading file '{}': {}", file_path, error);
                        }
                    }
                }
                continue;
            }
            _ => {}
        }

        // Execute the input as code
        execute_source(input, interpreter);
    }
}
