// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use crate::error::{ASError, ErrorKind, SourceLocation};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    Boolean(bool),
    
    // Keywords
    Let, Fn, If, Else, ElseIf, While, For, Do, 
    Break, Continue, Return, Output, Input, Import,
    
    // Operators
    Plus, Minus, Star, Slash, Percent, Caret,
    Eq, EqEq, Ne, Lt, Le, Gt, Ge,
    And, Or, Not, BitwiseAnd, BitwiseOr,
    LeftShift, RightShift,
    
    // Delimiters
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Comma, Semicolon, Colon,
    
    // Special
    Inc, Dec, EOF,
}

pub struct Lexer<'a> {
    #[allow(dead_code)]
    input: &'a str,
    chars: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ASError> {
        let mut tokens = Vec::new();
        
        while self.position < self.chars.len() {
            let ch = self.chars[self.position];
            
            match ch {
                // Whitespace
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance_line();
                }
                
                // Delimiters
                '(' => { tokens.push(Token::LParen); self.advance(); }
                ')' => { tokens.push(Token::RParen); self.advance(); }
                '{' => { tokens.push(Token::LBrace); self.advance(); }
                '}' => { tokens.push(Token::RBrace); self.advance(); }
                '[' => { tokens.push(Token::LBracket); self.advance(); }
                ']' => { tokens.push(Token::RBracket); self.advance(); }
                ',' => { tokens.push(Token::Comma); self.advance(); }
                ';' => { tokens.push(Token::Semicolon); self.advance(); }
                ':' => { tokens.push(Token::Colon); self.advance(); }
                
                // Operators
                '+' => {
                    if self.match_next('+') { tokens.push(Token::Inc); }
                    else { tokens.push(Token::Plus); }
                }
                '-' => {
                    if self.match_next('-') { tokens.push(Token::Dec); }
                    else { tokens.push(Token::Minus); }
                }
                '*' => { tokens.push(Token::Star); self.advance(); }
                '/' => {
                    if self.peek_next() == '/' {
                        self.skip_comment();
                    } else {
                        tokens.push(Token::Slash); self.advance();
                    }
                }
                '%' => { tokens.push(Token::Percent); self.advance(); }
                '^' => { tokens.push(Token::Caret); self.advance(); }
                '=' => {
                    if self.match_next('=') { tokens.push(Token::EqEq); }
                    else { tokens.push(Token::Eq); }
                }
                '!' => {
                    if self.match_next('=') { tokens.push(Token::Ne); }
                    else { tokens.push(Token::Not); }
                }
                '<' => {
                    if self.match_next('=') { tokens.push(Token::Le); }
                    else if self.match_next('<') { tokens.push(Token::LeftShift); }
                    else { tokens.push(Token::Lt); }
                }
                '>' => {
                    if self.match_next('=') { tokens.push(Token::Ge); }
                    else if self.match_next('>') { tokens.push(Token::RightShift); }
                    else { tokens.push(Token::Gt); }
                }
                '&' => {
                    if self.match_next('&') { tokens.push(Token::And); }
                    else { tokens.push(Token::BitwiseAnd); }
                }
                '|' => {
                    if self.match_next('|') { tokens.push(Token::Or); }
                    else { tokens.push(Token::BitwiseOr); }
                }
                
                // Strings
                '"' => tokens.push(self.read_string()?),
                
                // Numbers
                c if c.is_digit(10) => tokens.push(self.read_number()?),
                
                // Identifiers and keywords
                c if c.is_alphabetic() || c == '_' => tokens.push(self.read_identifier()),
                
                _ => {
                    return Err(ASError::new(
                        ErrorKind::SyntaxError,
                        format!("Unexpected character: {}", ch),
                        SourceLocation::new(self.line, self.column),
                    ));
                }
            }
        }
        
        tokens.push(Token::EOF);
        Ok(tokens)
    }
    
    fn advance(&mut self) {
        self.position += 1;
        self.column += 1;
    }
    
    fn advance_line(&mut self) {
        self.position += 1;
        self.line += 1;
        self.column = 1;
    }
    
    fn match_next(&mut self, expected: char) -> bool {
        if self.position + 1 >= self.chars.len() {
            return false;
        }
        if self.chars[self.position + 1] == expected {
            self.advance(); // consume current
            self.advance(); // consume next
            return true;
        }
        self.advance(); // consume current
        false
    }
    
    fn peek_next(&self) -> char {
        if self.position + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.position + 1]
        }
    }
    
    fn skip_comment(&mut self) {
        // Skip // comment until newline
        while self.position < self.chars.len() && self.chars[self.position] != '\n' {
            self.position += 1;
        }
        // Don't consume newline, let main loop handle it to increment line counter
    }
    
    fn read_string(&mut self) -> Result<Token, ASError> {
        let start_line = self.line;
        let start_col = self.column;
        
        self.advance(); // Skip opening quote
        let mut value = String::new();
        
        while self.position < self.chars.len() && self.chars[self.position] != '"' {
            if self.chars[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            value.push(self.chars[self.position]);
            self.position += 1;
        }
        
        if self.position >= self.chars.len() {
            return Err(ASError::new(
                ErrorKind::SyntaxError,
                "Unterminated string literal".to_string(),
                SourceLocation::new(start_line, start_col),
            ));
        }
        
        self.advance(); // Skip closing quote
        Ok(Token::String(value))
    }
    
    fn read_number(&mut self) -> Result<Token, ASError> {
        let mut value = String::new();
        let mut has_dot = false;
        
        while self.position < self.chars.len() {
            let ch = self.chars[self.position];
            if ch.is_digit(10) {
                value.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        match value.parse::<f64>() {
            Ok(num) => Ok(Token::Number(num)),
            Err(_) => Err(ASError::new(
                ErrorKind::SyntaxError,
                format!("Invalid number: {}", value),
                SourceLocation::new(self.line, self.column),
            )),
        }
    }
    
    fn read_identifier(&mut self) -> Token {
        let mut value = String::new();
        
        while self.position < self.chars.len() {
            let ch = self.chars[self.position];
            if ch.is_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        match value.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "elseif" => Token::ElseIf,
            "while" => Token::While,
            "for" => Token::For,
            "do" => Token::Do,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "return" => Token::Return,
            "output" => Token::Output,
            "input" => Token::Input,
            "import" => Token::Import,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            _ => Token::Identifier(value),
        }
    }
}
