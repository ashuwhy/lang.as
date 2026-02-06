// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use crate::lexer::{Lexer, Token};
use crate::error::{ASError, ErrorKind, SourceLocation};

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Subtract, Multiply, Divide, Modulo, Power,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or, BitwiseAnd, BitwiseOr, LeftShift, RightShift,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate, Not, BitwiseNot, Increment, Decrement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Array {
        elements: Vec<Expression>,
    },
    Index {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
    },
    UnaryOp {
        operator: UnaryOp,
        operand: Box<Expression>,
    },
    Grouping(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
    },
    Output(Expression),
    Input {
        prompt: Option<Expression>,
        target: String,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        elif_branches: Vec<(Expression, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Box<Statement>>,
        body: Vec<Statement>,
    },
    Break,
    Continue,
    Return(Option<Expression>),
    ExpressionStmt(Expression),
}

#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>,
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * / %
    Unary,      // ! -
    Call,       // . () []
    Primary,
}

#[allow(dead_code)]
pub struct Parser<'a> {
    #[allow(dead_code)]
    lexer: Lexer<'a>,
    #[allow(dead_code)]
    current_token: Token,
    #[allow(dead_code)]
    peek_token: Token,
    #[allow(dead_code)]
    location: SourceLocation,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.tokenize().unwrap_or(vec![Token::EOF]).first().cloned().unwrap_or(Token::EOF);
        // This is a bit simplified; real implementation should use iterator
        // For now, let's just reuse the simpler vector approach from before but adapted
        // Actually, let's rewrite to token iterator to be cleaner or just load all tokens
        Parser {
            lexer: Lexer::new(input), // We'll just re-create lexer for now or change design
            current_token: Token::EOF,
            peek_token: Token::EOF,
            location: SourceLocation::new(0, 0),
        }
    }
    
    // Better approach: consume tokens from a vector
    pub fn parse(input: &'a str) -> Result<AST, ASError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        let mut parser = ParserInstance::new(tokens);
        parser.parse()
    }
}

struct ParserInstance {
    tokens: Vec<Token>,
    current: usize,
}

impl ParserInstance {
    fn new(tokens: Vec<Token>) -> Self {
        ParserInstance {
            tokens,
            current: 0,
        }
    }

    fn parse(&mut self) -> Result<AST, ASError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(AST { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ASError> {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::Output => self.parse_output(),
            Token::Input => self.parse_input(),
            Token::Fn => self.parse_function(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Break => {
                self.advance();
                self.consume_semicolon()?;
                Ok(Statement::Break)
            }
            Token::Continue => {
                self.advance();
                self.consume_semicolon()?;
                Ok(Statement::Continue)
            }
            Token::Return => self.parse_return(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let(&mut self) -> Result<Statement, ASError> {
        self.advance(); // consume let
        
        let name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err(self.error("Expected variable name")),
        };
        
        if self.advance() != Token::Eq {
            return Err(self.error("Expected '=' after variable name"));
        }
        
        let value = self.parse_expression(Precedence::None)?;
        self.consume_semicolon()?;
        
        Ok(Statement::Let { name, value })
    }

    fn parse_output(&mut self) -> Result<Statement, ASError> {
        self.advance(); // consume output
        let expr = self.parse_expression(Precedence::None)?;
        // Output doesn't enforce semicolon in original design, but strict parser should
        if self.peek() == Token::Semicolon {
            self.advance();
        }
        Ok(Statement::Output(expr))
    }
    
    fn parse_input(&mut self) -> Result<Statement, ASError> {
        self.advance(); // consume input
        
        let mut prompt = None;
        // Check if there is a prompt string
        if let Token::String(_) = self.peek() {
            if let Token::String(s) = self.advance() {
                prompt = Some(Expression::String(s));
            }
        }
        
        // Maybe "into" keyword? Original syntax: input "Prompt" into var
        // Or simplified: input "Prompt" var
        // Use implicit syntax for now
        
        let target = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err(self.error("Expected variable name for input target")),
        };
        
        self.consume_semicolon()?;
        Ok(Statement::Input { prompt, target })
    }

    fn parse_function(&mut self) -> Result<Statement, ASError> {
        self.advance(); // consume fn
        
        let name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err(self.error("Expected function name")),
        };
        
        if self.advance() != Token::LParen {
            return Err(self.error("Expected '(' after function name"));
        }
        
        let mut params = Vec::new();
        if self.peek() != Token::RParen {
            loop {
                match self.advance() {
                    Token::Identifier(s) => params.push(s),
                    _ => return Err(self.error("Expected parameter name")),
                }
                
                if self.peek() == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        if self.advance() != Token::RParen {
            return Err(self.error("Expected ')' after parameters"));
        }
        
        if self.advance() != Token::LBrace {
            return Err(self.error("Expected '{' before function body"));
        }
        
        let body = self.parse_block()?;
        
        Ok(Statement::Function { name, params, body })
    }
    
    fn parse_if(&mut self) -> Result<Statement, ASError> {
        self.advance(); // consume if
        let condition = self.parse_expression(Precedence::None)?;
        
        if self.advance() != Token::LBrace {
            return Err(self.error("Expected '{' after if condition"));
        }
        
        let then_branch = self.parse_block()?;
        let mut elif_branches = Vec::new();
        let mut else_branch = None;
        
        while self.peek() == Token::ElseIf {
            self.advance();
            let elif_cond = self.parse_expression(Precedence::None)?;
            if self.advance() != Token::LBrace {
                return Err(self.error("Expected '{' after elseif condition"));
            }
            elif_branches.push((elif_cond, self.parse_block()?));
        }
        
        if self.peek() == Token::Else {
            self.advance();
            if self.advance() != Token::LBrace {
                return Err(self.error("Expected '{' after else"));
            }
            else_branch = Some(self.parse_block()?);
        }
        
        Ok(Statement::If { condition, then_branch, elif_branches, else_branch })
    }
    
    fn parse_while(&mut self) -> Result<Statement, ASError> {
        self.advance(); // consume while
        let condition = self.parse_expression(Precedence::None)?;
        
        if self.advance() != Token::LBrace {
            return Err(self.error("Expected '{' after while condition"));
        }
        
        let body = self.parse_block()?;
        Ok(Statement::While { condition, body })
    }
    
    fn parse_for(&mut self) -> Result<Statement, ASError> {
        // for (init; cond; update) { ... }
        self.advance(); // consume for
        if self.advance() != Token::LParen {
            return Err(self.error("Expected '(' after for"));
        }
        
        let init = if self.peek() == Token::Semicolon {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };
        // parse_statement consumes the semicolon if it's let/expr_stmt
        // but if it was just a semicolon (empty init), we need to consume it
        if init.is_none() {
            self.advance(); // consume ;
        }
        
        let condition = if self.peek() == Token::Semicolon {
            None
        } else {
            Some(self.parse_expression(Precedence::None)?)
        };
        self.consume_semicolon()?;
        
        let update = if self.peek() == Token::RParen {
            None
        } else {
            Some(Box::new(self.parse_expression_statement()?))
        };
        
        if self.advance() != Token::RParen {
            return Err(self.error("Expected ')' after for clauses"));
        }
        
        if self.advance() != Token::LBrace {
            return Err(self.error("Expected '{'"));
        }
        
        let body = self.parse_block()?;
        
        Ok(Statement::For { init, condition, update, body })
    }
    
    fn parse_return(&mut self) -> Result<Statement, ASError> {
        self.advance(); // consume return
        let value = if self.peek() == Token::Semicolon {
            None
        } else {
            Some(self.parse_expression(Precedence::None)?)
        };
        self.consume_semicolon()?;
        Ok(Statement::Return(value))
    }
    
    fn parse_block(&mut self) -> Result<Vec<Statement>, ASError> {
        let mut statements = Vec::new();
        while self.peek() != Token::RBrace && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        if self.advance() != Token::RBrace {
            return Err(self.error("Expected '}'"));
        }
        Ok(statements)
    }
    
    fn parse_expression_statement(&mut self) -> Result<Statement, ASError> {
        let expr = self.parse_expression(Precedence::None)?;
        self.consume_semicolon()?;
        Ok(Statement::ExpressionStmt(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ASError> {
        let mut left = self.parse_prefix()?;
        
        while precedence < self.get_precedence(self.peek()) {
            left = self.parse_infix(left)?;
        }
        
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expression, ASError> {
        let token = self.advance();
        match token {
            Token::Number(n) => Ok(Expression::Number(n)),
            Token::String(s) => Ok(Expression::String(s)),
            Token::Boolean(b) => Ok(Expression::Boolean(b)),
            Token::Identifier(s) => Ok(Expression::Identifier(s)),
            Token::LParen => {
                let expr = self.parse_expression(Precedence::None)?;
                if self.advance() != Token::RParen {
                    return Err(self.error("Expected ')'"));
                }
                Ok(Expression::Grouping(Box::new(expr)))
            }
            Token::LBracket => self.parse_array(),
            Token::Minus => self.parse_unary(UnaryOp::Negate),
            Token::Not => self.parse_unary(UnaryOp::Not),
            _ => Err(self.error(&format!("Expected expression, found {:?}", token))),
        }
    }
    
    fn parse_unary(&mut self, op: UnaryOp) -> Result<Expression, ASError> {
        let operand = self.parse_expression(Precedence::Unary)?;
        Ok(Expression::UnaryOp { operator: op, operand: Box::new(operand) })
    }
    
    fn parse_array(&mut self) -> Result<Expression, ASError> {
        let mut elements = Vec::new();
        if self.peek() != Token::RBracket {
            loop {
                elements.push(self.parse_expression(Precedence::None)?);
                if self.peek() == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        if self.advance() != Token::RBracket {
            return Err(self.error("Expected ']'"));
        }
        
        Ok(Expression::Array { elements })
    }

    fn parse_infix(&mut self, left: Expression) -> Result<Expression, ASError> {
        let token = self.advance();
        match token {
            Token::Plus => self.binary(left, BinaryOp::Add),
            Token::Minus => self.binary(left, BinaryOp::Subtract),
            Token::Star => self.binary(left, BinaryOp::Multiply),
            Token::Slash => self.binary(left, BinaryOp::Divide),
            Token::Percent => self.binary(left, BinaryOp::Modulo),
            Token::EqEq => self.binary(left, BinaryOp::Eq),
            Token::Ne => self.binary(left, BinaryOp::Ne),
            Token::Lt => self.binary(left, BinaryOp::Lt),
            Token::Le => self.binary(left, BinaryOp::Le),
            Token::Gt => self.binary(left, BinaryOp::Gt),
            Token::Ge => self.binary(left, BinaryOp::Ge),
            Token::And => self.binary(left, BinaryOp::And),
            Token::Or => self.binary(left, BinaryOp::Or),
            Token::LParen => self.call(left),
            Token::LBracket => self.index(left),
            _ => Err(self.error("Unknown infix operator")),
        }
    }
    
    fn binary(&mut self, left: Expression, op: BinaryOp) -> Result<Expression, ASError> {
        let precedence = self.get_precedence(self.prev());
        let right = self.parse_expression(precedence)?;
        Ok(Expression::BinaryOp { left: Box::new(left), operator: op, right: Box::new(right) })
    }
    
    fn call(&mut self, function: Expression) -> Result<Expression, ASError> {
        let mut arguments = Vec::new();
        if self.peek() != Token::RParen {
            loop {
                arguments.push(self.parse_expression(Precedence::None)?);
                if self.peek() == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        if self.advance() != Token::RParen {
            return Err(self.error("Expected ')'"));
        }
        
        Ok(Expression::Call { function: Box::new(function), arguments })
    }
    
    fn index(&mut self, array: Expression) -> Result<Expression, ASError> {
        let index = self.parse_expression(Precedence::None)?;
        if self.advance() != Token::RBracket {
            return Err(self.error("Expected ']'"));
        }
        Ok(Expression::Index { array: Box::new(array), index: Box::new(index) })
    }
    
    fn prev(&self) -> Token {
        // Limitation of our simple vector parser, but tokens vec is available 
        // Logic should be cleaner in real iter implementation
        // For now hack:
        self.tokens[self.current - 1].clone()
    }

    fn get_precedence(&self, token: Token) -> Precedence {
        match token {
            Token::Eq | Token::EqEq | Token::Ne => Precedence::Equality,
            Token::Lt | Token::Le | Token::Gt | Token::Ge => Precedence::Comparison,
            Token::Plus | Token::Minus => Precedence::Term,
            Token::Star | Token::Slash | Token::Percent => Precedence::Factor,
            Token::And => Precedence::And,
            Token::Or => Precedence::Or,
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Call,
            _ => Precedence::None,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        if self.current >= self.tokens.len() {
            return Token::EOF;
        }
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Token::EOF => true,
            _ => false,
        }
    }
    
    fn consume_semicolon(&mut self) -> Result<(), ASError> {
        if self.peek() == Token::Semicolon {
            self.advance();
            Ok(())
        } else if self.peek() == Token::EOF || self.peek() == Token::RBrace {
            // Optional semicolon at end of block/file
            Ok(())
        } else {
            // For now, make semicolons optional to be friendlier like Python usually
            // but strict parsing requires them. Let's make them optional.
            Ok(())
        }
    }
    
    fn error(&self, message: &str) -> ASError {
        ASError::new(
            ErrorKind::SyntaxError,
            message.to_string(),
            SourceLocation::new(0, 0), // ToDo: propagating location from Token
        )
    }
}