use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Vec<Stmt>),
    Return(Token, Option<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        // Skip any leading newlines
        self.skip_newlines();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => return Err(err),
            }

            // Skip any newlines between statements
            self.skip_newlines();
        }
        Ok(statements)
    }

    fn skip_newlines(&mut self) {
        while self.check(TokenType::Newline) {
            self.advance();
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();

        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }
        if self.match_token(&[TokenType::Fun]) {
            return self.function("function");
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier("".to_string()), "Expect variable name.")?;

        self.skip_newlines();
        let initializer = if self.match_token(&[TokenType::Equal]) {
            self.skip_newlines();
            Some(self.expression()?)
        } else {
            None
        };

        self.skip_newlines();

        // Handle both semicolon and newline termination
        if !self.match_token(&[TokenType::Semicolon]) && !self.match_token(&[TokenType::Newline]) {
            return Err("Expect ';' or newline after variable declaration.".to_string());
        }

        Ok(Stmt::Var(name, initializer))
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier("".to_string()), &format!("Expect {} name.", kind))?;
        self.consume(TokenType::LeftParen, &format!("Expect '(' after {} name.", kind))?;

        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err("Cannot have more than 255 parameters.".to_string());
                }
                parameters.push(self.consume(TokenType::Identifier("".to_string()), "Expect parameter name.")?);
                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(TokenType::LeftBrace, &format!("Expect '{{' before {} body.", kind))?;

        let body = self.block()?;
        Ok(Stmt::Function(name, parameters, body))
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();

        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        if !self.match_token(&[TokenType::Semicolon]) && !self.match_token(&[TokenType::Newline]) {
            return Err("Expect ';' or newline after value.".to_string());
        }
        Ok(Stmt::Print(value))
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While(condition, body))
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let keyword = self.previous().clone();
        let value = if !self.check(TokenType::Semicolon) && !self.check(TokenType::Newline) {
            Some(self.expression()?)
        } else {
            None
        };
        if !self.match_token(&[TokenType::Semicolon]) && !self.match_token(&[TokenType::Newline]) {
            return Err("Expect ';' or newline after return value.".to_string());
        }
        Ok(Stmt::Return(keyword, value))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        self.skip_newlines();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => return Err(err),
            }

            self.skip_newlines();
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        if !self.match_token(&[TokenType::Semicolon]) && !self.match_token(&[TokenType::Newline]) {
            return Err("Expect ';' or newline after expression.".to_string());
        }
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.skip_newlines();
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        self.skip_newlines();
        let expr = self.equality()?;

        self.skip_newlines();
        if self.match_token(&[TokenType::Equal]) {
            self.skip_newlines();
            let _equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }
            return Err("Invalid assignment target.".to_string());
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        self.skip_newlines();

        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.call()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if let TokenType::Number(n) = self.peek().token_type.clone() {
            self.advance();
            return Ok(Expr::Literal(Literal::Number(n)));
        }
        if let TokenType::String(s) = self.peek().token_type.clone() {
            self.advance();
            return Ok(Expr::Literal(Literal::String(s)));
        }
        if self.check(TokenType::Identifier("".to_string())) {
            self.advance();
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.match_token(&[TokenType::LeftParen]) {
            self.skip_newlines();
            let expr = self.expression()?;
            self.skip_newlines();
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err("Expect expression.".to_string())
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LeftParen]) {
                self.skip_newlines();
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = Vec::new();

        self.skip_newlines();

        if !self.check(TokenType::RightParen) {
            loop {
                self.skip_newlines();
                arguments.push(self.expression()?);
                self.skip_newlines();

                if !self.match_token(&[TokenType::Comma]) {
                    break;
                }

                self.skip_newlines();
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call(Box::new(callee), paren, arguments))
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(message.to_string())
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        match (&self.peek().token_type, token_type) {
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (a, b) => a == &b,
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
