use rustjs::{Lexer, Parser, Interpreter};

fn main() {
    // Example source code
    let source = r#"
        var greeting = "Hello, World!";
        print greeting;

        // Function definition
        fun add(a, b) {
            return a + b;
        }

        // Using the function
        var result = add(5, 3);
        print result;

        // Conditional statement
        if (result > 5) {
            print "Result is greater than 5";
        } else {
            print "Result is less than or equal to 5";
        }

        // Loop example
        var counter = 0;
        while (counter < 3) {
            print counter;
            counter = counter + 1;
        }
    "#;

    // Create lexer and tokenize
    let mut lexer = Lexer::new(source.to_string());
    match lexer.scan_tokens() {
        Ok(tokens) => {
            // Debug: Print tokens
            println!("Tokens:");
            for (i, token) in tokens.iter().enumerate() {
                println!("{}: {:?}", i, token);
            }

            // Create parser and parse
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(statements) => {
                    println!("Successfully parsed {} statements", statements.len());

                    // Create interpreter and execute
                    let mut interpreter = Interpreter::new();
                    if let Err(error) = interpreter.interpret(statements) {
                        println!("Runtime error: {}", error);
                    }
                }
                Err(error) => {
                    println!("Parse error: {}", error);
                    // Print the current token for debugging
                    println!("ERROR OCCURRED AT TOKEN NUMBER: {}", parser.current);
                }
            }
        }
        Err(error) => println!("Lexical error: {}", error),
    }
}
