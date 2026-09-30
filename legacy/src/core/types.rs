// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use crate::parser::{AST, Statement, Expression, BinaryOp, UnaryOp, Parser};
use crate::error::{ASError, ErrorKind, SourceLocation};
use crate::resolver::Resolver;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Array(Box<Type>),
    Function {
        params: Vec<Type>,
        returns: Box<Type>,
    },
    Any,
    Void,
    Unknown,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Number => write!(f, "Number"),
            Type::String => write!(f, "String"),
            Type::Boolean => write!(f, "Boolean"),
            Type::Array(inner) => write!(f, "Array<{}>", inner),
            Type::Function { params, returns } => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", returns)
            }
            Type::Any => write!(f, "Any"),
            Type::Void => write!(f, "Void"),
            Type::Unknown => write!(f, "Unknown"),
        }
    }
}

pub struct TypeChecker {
    variables: HashMap<String, Type>,
    functions: HashMap<String, Type>,
    resolver: Resolver,
    visited: HashSet<PathBuf>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            variables: HashMap::new(),
            functions: HashMap::new(),
            resolver: Resolver::new(),
            visited: HashSet::new(),
        }
    }

    pub fn check(&mut self, ast: &AST) -> Result<(), ASError> {
        for statement in &ast.statements {
            self.check_statement(statement)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), ASError> {
        match stmt {
            Statement::Let { name, value, type_annotation } => {
                let inferred = self.infer_type(value)?;
                
                if let Some(annotated) = type_annotation {
                    if !self.types_compatible(annotated, &inferred) {
                        return Err(self.error(&format!(
                            "Type mismatch: expected {}, got {}",
                            annotated, inferred
                        )));
                    }
                    self.variables.insert(name.clone(), annotated.clone());
                } else {
                    self.variables.insert(name.clone(), inferred);
                }
                Ok(())
            }
            Statement::Output(expr) => {
                self.infer_type(expr)?;
                Ok(())
            }
            Statement::If { condition, then_branch, elif_branches, else_branch } => {
                let cond_type = self.infer_type(condition)?;
                if cond_type != Type::Boolean && cond_type != Type::Any {
                    return Err(self.error(&format!(
                        "If condition must be Boolean, got {}",
                        cond_type
                    )));
                }
                
                for stmt in then_branch {
                    self.check_statement(stmt)?;
                }
                
                for (elif_cond, elif_body) in elif_branches {
                    let elif_type = self.infer_type(elif_cond)?;
                    if elif_type != Type::Boolean && elif_type != Type::Any {
                        return Err(self.error("Elif condition must be Boolean"));
                    }
                    for stmt in elif_body {
                        self.check_statement(stmt)?;
                    }
                }
                
                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.check_statement(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                let cond_type = self.infer_type(condition)?;
                if cond_type != Type::Boolean && cond_type != Type::Any {
                    return Err(self.error("While condition must be Boolean"));
                }
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                Ok(())
            }
            Statement::Function { name, params, body, return_type } => {
                // Create function signature
                let param_types = params.iter().map(|_| Type::Any).collect();
                let ret_type = return_type.clone().unwrap_or(Type::Any);
                
                self.functions.insert(name.clone(), Type::Function {
                    params: param_types,
                    returns: Box::new(ret_type),
                });
                
                // Type check body (simplified - doesn't add params to scope)
                for stmt in body {
                    self.check_statement(stmt)?;
                }
                Ok(())
            }
            Statement::Import { path } => {
                let resolved = self.resolver.resolve(path, None).map_err(|e| self.error(&format!("Import failed: {}", e)))?;
                if self.visited.contains(&resolved) { return Ok(()); }
                self.visited.insert(resolved.clone());
                
                let source = self.resolver.read_file(&resolved).map_err(|e| self.error(&format!("Read failed: {}", e)))?;
                // Parse the imported file
                let ast = Parser::parse(&source)?;
                
                // Recursively check the imported AST
                // Variables defined in the imported file will be added to self.variables
                self.check(&ast)?;
                
                Ok(())
            }
            _ => Ok(()), // Other statements pass through
        }
    }

    fn infer_type(&self, expr: &Expression) -> Result<Type, ASError> {
        match expr {
            Expression::Number(_) => Ok(Type::Number),
            Expression::String(_) => Ok(Type::String),
            Expression::Boolean(_) => Ok(Type::Boolean),
            Expression::Identifier(name) => {
                self.variables.get(name)
                    .cloned()
                    .ok_or_else(|| self.error(&format!("Undefined variable: {}", name)))
            }
            Expression::BinaryOp { left, operator, right } => {
                let left_type = self.infer_type(left)?;
                let right_type = self.infer_type(right)?;
                
                match operator {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | 
                    BinaryOp::Divide | BinaryOp::Modulo | BinaryOp::Power => {
                        if left_type == Type::Number && right_type == Type::Number {
                            Ok(Type::Number)
                        } else if left_type == Type::String && right_type == Type::String && *operator == BinaryOp::Add {
                            Ok(Type::String)
                        } else {
                            Err(self.error(&format!(
                                "Cannot apply {:?} to {} and {}",
                                operator, left_type, right_type
                            )))
                        }
                    }
                    BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | 
                    BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                        Ok(Type::Boolean)
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type == Type::Boolean && right_type == Type::Boolean {
                            Ok(Type::Boolean)
                        } else {
                            Err(self.error("Logical operators require Boolean operands"))
                        }
                    }
                    _ => Ok(Type::Any),
                }
            }
            Expression::UnaryOp { operator, operand } => {
                let operand_type = self.infer_type(operand)?;
                match operator {
                    UnaryOp::Negate => {
                        if operand_type == Type::Number {
                            Ok(Type::Number)
                        } else {
                            Err(self.error("Cannot negate non-number"))
                        }
                    }
                    UnaryOp::Not => {
                        if operand_type == Type::Boolean {
                            Ok(Type::Boolean)
                        } else {
                            Err(self.error("Cannot apply 'not' to non-boolean"))
                        }
                    }
                    _ => Ok(operand_type),
                }
            }
            Expression::Call { function, arguments: _ } => {
                if let Expression::Identifier(name) = &**function {
                    if let Some(Type::Function { returns, .. }) = self.functions.get(name) {
                        Ok(*returns.clone())
                    } else {
                        Ok(Type::Any) // Built-in or unknown function
                    }
                } else {
                    Ok(Type::Any)
                }
            }
            Expression::Array { elements } => {
                if elements.is_empty() {
                    Ok(Type::Array(Box::new(Type::Any)))
                } else {
                    let first_type = self.infer_type(&elements[0])?;
                    Ok(Type::Array(Box::new(first_type)))
                }
            }
            _ => Ok(Type::Any),
        }
    }

    fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        if expected == &Type::Any || actual == &Type::Any {
            return true;
        }
        expected == actual
    }

    fn error(&self, msg: &str) -> ASError {
        ASError::new(ErrorKind::TypeError, msg.to_string(), SourceLocation::new(0, 0))
    }
}
