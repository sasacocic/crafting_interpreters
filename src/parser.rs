use crate::LoxErrors;
use log::{debug, trace};
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

// honestly just using this as a placeholder for now
#[derive(Debug)]
pub struct Object(pub String);

impl ToString for Object {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<&str> for Object {
    fn from(string: &str) -> Self {
        Object(string.to_string())
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn _new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Object>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lexeme.ends_with("\n") {
            write!(
                f,
                "_{:?}_{}_{}",
                &self.token_type,
                &self
                    .lexeme
                    .get(0..self.lexeme.len() - 1)
                    .expect("unwrap lexeme without a newline"),
                self.line
            )
        } else {
            write!(f, "_{:?}_{}_{}", &self.token_type, &self.lexeme, self.line)
        }
    }
}

#[derive(Default, Debug)]
pub struct Scanner {
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>,
    pub source: String,
}

// todo implemet iter on scanner
impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            ..Default::default()
        }
    }

    fn is_at_end(&self) -> bool {
        // probably shouldn't do as i32 here, because not every computer will be i32
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        // seems kinda crazy to always create this chars iterator, but whatever
        let op = self.source.chars().nth(self.current);
        self.current += 1;
        trace!(
            "scanner advanced to character: {:?}",
            self.source.chars().nth(self.current)
        );
        op
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        let end = if self.start == self.current {
            self.current + 1
        } else {
            self.current
        };
        let text = self.source.get(self.start..end);
        match text {
            Some(text) => {
                debug!("lexme being added to token: {}", text);
                self.tokens.push(Token {
                    lexeme: text.to_string(),
                    line: self.line,
                    literal,
                    token_type,
                })
            }
            None => {
                panic!(
                    r#"coudn't add token - error because I couldn't read the proper substring from:
                    self.source.get(self.start..self.current+1) - self.start: {}, self.current+1{}"#,
                    self.start,
                    self.current + 1
                );
                // panic-ing right now. Should be throwing an error instead though.
            }
        }
    }

    fn match_next(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        let substring_match = self.source.get(self.current..self.current + 1);
        trace!(
            "checking if {:?} != Some({:?})",
            &substring_match,
            Some(expected)
        );
        if substring_match != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0'); // what is \0 - null terminator? end of file?
        }
        return self.source.chars().nth(self.current);
    }

    // TODO: should be returning an error if something goes wrong
    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // need to throw an error here
            panic!(
                "trying to create a string token, but we are at the end of the file / expression"
            );
        }

        self.advance();

        let text = self.source.get(self.start + 1..self.current - 1);
        trace!("string read in 'string()': {:?}", text);
        self.add_token(
            TokenType::STRING,
            Some(Object(
                text.expect("string literal to have been read").to_string(),
            )),
        )
    }

    fn is_digit(&mut self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source
                .chars()
                .nth(self.current + 1)
                .expect("nth char to exist")
        }
    }

    fn number(&mut self) {
        while self.is_digit(self.peek().expect("char should be returned from peek")) {
            self.advance();
        }

        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek().expect("peek to return a char")) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::NUMBER,
            self.source.get(self.start..self.current).map(|a| a.into()),
        )
    }

    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alphanumeric(&mut self, character: Option<char>) -> bool {
        let character =
            character.expect("character should be present if checking of an alphanumeric");
        self.is_alpha(character) || self.is_digit(character)
    }

    fn identifier(&mut self) -> Result<(), Box<dyn Error>> {
        trace!("found an identifier");
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::AND);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("flase", TokenType::FALSE);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("fun", TokenType::FUN);
        keywords.insert("if", TokenType::IF);
        keywords.insert("nil", TokenType::NIL);
        keywords.insert("or", TokenType::OR);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("super", TokenType::SUPER);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("true", TokenType::TRUE);
        keywords.insert("var", TokenType::VAR);
        keywords.insert("while", TokenType::WHILE);

        let text = self
            .source
            .get(self.start..self.current)
            .expect("couldn't get substring from source");
        let token_type = keywords.get(text);

        trace!("found identifier: {}", text);
        match token_type {
            None => {
                self.add_token(TokenType::IDENTIFIER, Some(text.into()));
            }
            Some(token_type) => {
                self.add_token(token_type.clone(), Some(text.into()));
            }
        }

        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), Box<dyn Error>> {
        let c = self
            .advance()
            .ok_or("should be able to advance w/o any error")?;
        match c {
            '(' => self.add_token(TokenType::LeftParen, Some(Object("(".to_string()))),
            ')' => self.add_token(TokenType::RightParen, Some(Object(")".to_string()))),
            '{' => self.add_token(TokenType::LeftBrace, Some(Object("{".to_string()))),
            '}' => self.add_token(TokenType::RightBrace, Some("}".into())),
            ',' => self.add_token(TokenType::COMMA, Some(",".into())),
            '.' => self.add_token(TokenType::DOT, Some(".".into())),
            '-' => self.add_token(TokenType::MINUS, Some("-".into())),
            '+' => self.add_token(TokenType::PLUS, Some("+".into())),
            ';' => self.add_token(TokenType::SEMICOLON, Some(";".into())),
            '*' => self.add_token(TokenType::STAR, Some("*".into())),
            '!' => {
                let b = self.match_next("=");
                let (tt, object) = if b {
                    (TokenType::BangEqual, Some("!=".into()))
                } else {
                    (TokenType::BANG, Some("!".into()))
                };
                self.add_token(tt, object)
            }
            '=' => {
                let b = self.match_next("=");
                let (tt, object) = if b {
                    (TokenType::EqualEqual, Some("==".into()))
                } else {
                    (TokenType::EQUAL, Some("=".into()))
                };
                self.add_token(tt, object)
            }
            '<' => {
                let b = self.match_next("=");
                let (tt, object) = if b {
                    (TokenType::LessEqual, Some("<=".into()))
                } else {
                    (TokenType::LESS, Some("<".into()))
                };
                self.add_token(tt, object);
            }
            '>' => {
                let b = self.match_next("=");
                let (tt, object) = if b {
                    (TokenType::GreaterEqual, Some(">=".into()))
                } else {
                    (TokenType::GREATER, Some("!".into()))
                };
                self.add_token(tt, object);
            }
            '/' => {
                let matched = self.match_next("/");
                if matched {
                    // it's a comment so just skip over the list
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, Some("/".into()));
                };
            }
            ' ' | '\r' | '\t' => {
                trace!("NOOP - '','\\r' or '\\t'");
                // do nothing
            }
            '\n' => {
                self.line += 1;
                trace!("new line - incrementing parser.line: {}", self.line);
            }
            '"' => {
                trace!("matched on self.string()");
                self.string()
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier()?
                } else {
                    // throw an error
                    eprintln!("couldn't find the lexeme you're tyring to do");
                    return Err(Box::new(LoxErrors::GeneralError(
                        self.line,
                        "Unexpected character".to_string(),
                    )));
                }
            }
        }

        Ok(())
    }

    // really this should return a result
    pub fn scan_tokens(&mut self) -> Result<&mut Vec<Token>, Vec<Box<dyn Error>>> {
        let mut errors: Vec<Box<dyn Error>> = Vec::new();

        trace!("scan_tokens - beginning");
        while !self.is_at_end() {
            trace!("is_at_end? {}", self.is_at_end());
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(error) => {
                    trace!("error in scan_token(): {}", error);
                    errors.push(error);
                }
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            line: self.line,
            literal: None,
        });

        if errors.is_empty() {
            trace!("no errors - returning tokens");
            Ok(&mut self.tokens)
        } else {
            Err(errors)
        }
    }
}
