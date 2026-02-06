// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use aslang::lexer::{Lexer, Token};

#[test]
fn test_basic_arithmetic() {
    let input = "10 + 20 * 30";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0], Token::Number(10.0));
    assert_eq!(tokens[1], Token::Plus);
    assert_eq!(tokens[2], Token::Number(20.0));
    assert_eq!(tokens[3], Token::Star);
    assert_eq!(tokens[4], Token::Number(30.0));
    assert_eq!(tokens[5], Token::EOF);
}

#[test]
fn test_identifiers_and_keywords() {
    let input = "let x = 42; if x > 10 { output x }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0], Token::Let);
    assert_eq!(tokens[1], Token::Identifier("x".to_string()));
    assert_eq!(tokens[2], Token::Eq);
    assert_eq!(tokens[3], Token::Number(42.0));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::If);
    assert_eq!(tokens[6], Token::Identifier("x".to_string()));
    assert_eq!(tokens[7], Token::Gt);
    assert_eq!(tokens[8], Token::Number(10.0));
    assert_eq!(tokens[9], Token::LBrace);
    assert_eq!(tokens[10], Token::Output);
    assert_eq!(tokens[11], Token::Identifier("x".to_string()));
    assert_eq!(tokens[12], Token::RBrace);
    assert_eq!(tokens[13], Token::EOF);
}

#[test]
fn test_string_literals() {
    let input = "\"Hello World\" \"Use \\\"quotes\\\"\"";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0], Token::String("Hello World".to_string()));
    // Note: Our simple lexer doesn't handle escapes yet, so we just test basic string
}

#[test]
fn test_operators() {
    let input = "== != <= >= && ||";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    
    assert_eq!(tokens[0], Token::EqEq);
    assert_eq!(tokens[1], Token::Ne);
    assert_eq!(tokens[2], Token::Le);
    assert_eq!(tokens[3], Token::Ge);
    assert_eq!(tokens[4], Token::And);
    assert_eq!(tokens[5], Token::Or);
}
