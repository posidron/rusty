use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token(";")]
    Semicolon,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,
    #[token("\n")]
    Newline,

    // One or two character tokens
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,

    // Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),
    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    // Keywords
    #[token("and")]
    And,
    #[token("class")]
    Class,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("for")]
    For,
    #[token("fun")]
    Fun,
    #[token("if")]
    If,
    #[token("nil")]
    Nil,
    #[token("or")]
    Or,
    #[token("print")]
    Print,
    #[token("return")]
    Return,
    #[token("super")]
    Super,
    #[token("this")]
    This,
    #[token("true")]
    True,
    #[token("var")]
    Var,
    #[token("while")]
    While,

    // Special tokens
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: {}, literal: {:?}, line: {}, column: {} }}",
            self.token_type, self.lexeme, self.literal, self.line, self.column
        )
    }
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            None,
            self.line,
            self.column,
        ));

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '\n' => {
                self.add_token(TokenType::Newline);
                self.line += 1;
                self.column = 1;
            }
            '"' => self.string()?,
            c if c.is_ascii_digit() => self.number()?,
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier()?,
            _ => return Err("Unexpected character.".to_string()),
        }
        Ok(())
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string.".to_string());
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String(value.clone()), Some(value));
        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token_with_literal(TokenType::Number(value.parse().unwrap()), Some(value));
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), String> {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(text.to_string()),
        };

        self.add_token(token_type);
        Ok(())
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        self.column += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.add_token_with_literal(token_type, Some(text));
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(
            token_type,
            text,
            literal,
            self.line,
            self.column,
        ));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
