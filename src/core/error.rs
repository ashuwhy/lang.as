// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        SourceLocation {
            line,
            column,
            file: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    SyntaxError,
    TypeError,
    RuntimeError,
    UndefinedVariable,
    UndefinedFunction,
    IOError,
}

#[derive(Debug, Clone)]
pub struct ASError {
    pub kind: ErrorKind,
    pub message: String,
    pub location: SourceLocation,
}

impl ASError {
    pub fn new(kind: ErrorKind, message: String, location: SourceLocation) -> Self {
        ASError {
            kind,
            message,
            location,
        }
    }
}

impl std::fmt::Display for ASError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?} at line {}, column {}: {}",
            self.kind, self.location.line, self.location.column, self.message
        )
    }
}
