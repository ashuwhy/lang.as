// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use crate::parser::{AST, Expression, Statement, BinaryOp, UnaryOp};
use crate::error::{ASError, ErrorKind, SourceLocation};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Opcode {
    LoadConst(f64),
    LoadString(String),
    LoadBool(bool),
    LoadVar(String),
    StoreVar(String),
    Call(String, usize),
    MakeArray(usize),
    GetIndex,
    SetIndex,
    Return,
    Output,
    Input,
    Import(String),
    
    // Arithmetic
    Add, Subtract, Multiply, Divide, Modulo, Power,
    
    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,
    
    // Logical
    And, Or, Not,
    
    // Unary
    Negate,
    
    // Control Flow
    Jump(usize),
    JumpIfFalse(usize),
    
    // Stack manipulation
    Pop,
}

pub struct Compiler {
    pub bytecode: Vec<Opcode>,
    variables: HashMap<String, usize>,
    functions: HashMap<String, usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            bytecode: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn compile(&mut self, ast: &AST) -> Result<Vec<Opcode>, ASError> {
        self.bytecode.clear();
        
        for statement in &ast.statements {
            self.compile_statement(statement)?;
        }
        
        Ok(self.bytecode.clone())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), ASError> {
        match statement {
            Statement::Let { name, value, type_annotation: _ } => {
                self.compile_expression(value)?;
                self.bytecode.push(Opcode::StoreVar(name.clone()));
                self.variables.insert(name.clone(), self.variables.len());
            }
            Statement::Output(expr) => {
                self.compile_expression(expr)?;
                self.bytecode.push(Opcode::Output);
            }
            Statement::Input { prompt, target } => {
                if let Some(p) = prompt {
                    self.compile_expression(p)?;
                    self.bytecode.push(Opcode::Output); // Print prompt
                }
                self.bytecode.push(Opcode::Input);
                self.bytecode.push(Opcode::StoreVar(target.clone()));
                self.variables.insert(target.clone(), self.variables.len());
            }
            Statement::Import { path } => {
                self.bytecode.push(Opcode::Import(path.clone()));
            }
            Statement::ExpressionStmt(expr) => {
                self.compile_expression(expr)?;
                self.bytecode.push(Opcode::Pop);
            }
            Statement::Function { name, params, body, return_type: _ } => {
                // ToDo: Function compilation needs jump over body or separate code segments
                // For simplicity now, we'll put it in main stream but need a jump over it
                // A better approach is to compile functions separately
                
                let jump_over = self.emit_jump(Opcode::Jump(0));
                
                let start_pos = self.bytecode.len();
                self.functions.insert(name.clone(), start_pos);
                
                // Add parameters to variables scope (simplified)
                for param in params {
                    self.variables.insert(param.clone(), self.variables.len());
                }
                
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                // Ensure implicit return
                self.bytecode.push(Opcode::LoadConst(0.0)); // Default return
                self.bytecode.push(Opcode::Return);
                
                self.patch_jump(jump_over);
            }
            Statement::If { condition, then_branch, elif_branches, else_branch } => {
                let mut exit_jumps = Vec::new();
                
                // 1. Compile 'if'
                self.compile_expression(condition)?;
                let mut jump_to_next = self.emit_jump(Opcode::JumpIfFalse(0));
                
                self.compile_block(then_branch)?;
                exit_jumps.push(self.emit_jump(Opcode::Jump(0)));
                
                self.patch_jump(jump_to_next);
                
                // 2. Compile 'elif's
                for (elif_cond, elif_body) in elif_branches {
                    self.compile_expression(elif_cond)?;
                    jump_to_next = self.emit_jump(Opcode::JumpIfFalse(0));
                    
                    self.compile_block(elif_body)?;
                    exit_jumps.push(self.emit_jump(Opcode::Jump(0)));
                    
                    self.patch_jump(jump_to_next);
                }
                
                // 3. Compile 'else'
                if let Some(else_stmts) = else_branch {
                    self.compile_block(else_stmts)?;
                }
                
                // 4. Patch all jumps to end
                let end_pos = self.bytecode.len();
                for jump in exit_jumps {
                    // Manually patch to end_pos
                     match &mut self.bytecode[jump] {
                        Opcode::Jump(ref mut val) => *val = end_pos,
                        _ => panic!("Expected Jump opcode"),
                    }
                }
            }
            Statement::While { condition, body } => {
                let loop_start = self.bytecode.len();
                self.compile_expression(condition)?;
                
                let jump_out = self.emit_jump(Opcode::JumpIfFalse(0));
                
                self.compile_block(body)?;
                self.emit_loop(loop_start);
                
                self.patch_jump(jump_out);
            }
            Statement::For { init, condition, update, body } => {
                if let Some(init_stmt) = init {
                    self.compile_statement(init_stmt)?;
                }
                
                let loop_start = self.bytecode.len();
                
                let mut jump_out = None;
                if let Some(cond) = condition {
                    self.compile_expression(cond)?;
                    jump_out = Some(self.emit_jump(Opcode::JumpIfFalse(0)));
                }
                
                self.compile_block(body)?;
                
                if let Some(upd) = update {
                    self.compile_statement(upd)?;
                }
                
                self.emit_loop(loop_start);
                
                if let Some(jump) = jump_out {
                    self.patch_jump(jump);
                }
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.compile_expression(e)?;
                } else {
                    self.bytecode.push(Opcode::LoadConst(0.0)); // Null/Void
                }
                self.bytecode.push(Opcode::Return);
            }
            _ => return Err(self.error("Statement not yet implemented in compiler")),
        }
        Ok(())
    }
    
    fn compile_block(&mut self, statements: &Vec<Statement>) -> Result<(), ASError> {
        for stmt in statements {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }

    fn compile_expression(&mut self, expr: &Expression) -> Result<(), ASError> {
        match expr {
            Expression::Number(n) => self.bytecode.push(Opcode::LoadConst(*n)),
            Expression::String(s) => self.bytecode.push(Opcode::LoadString(s.clone())),
            Expression::Boolean(b) => self.bytecode.push(Opcode::LoadBool(*b)),
            Expression::Identifier(name) => {
                // In real compiler we check if it exists or generic load
                self.bytecode.push(Opcode::LoadVar(name.clone()));
            },
            Expression::BinaryOp { left, operator, right } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                match operator {
                    BinaryOp::Add => self.bytecode.push(Opcode::Add),
                    BinaryOp::Subtract => self.bytecode.push(Opcode::Subtract),
                    BinaryOp::Multiply => self.bytecode.push(Opcode::Multiply),
                    BinaryOp::Divide => self.bytecode.push(Opcode::Divide),
                    BinaryOp::Eq => self.bytecode.push(Opcode::Eq),
                    BinaryOp::Lt => self.bytecode.push(Opcode::Lt),
                    BinaryOp::Gt => self.bytecode.push(Opcode::Gt),
                    _ => return Err(self.error("Binary operator not implemented")),
                }
            },
            Expression::UnaryOp { operator, operand } => {
                self.compile_expression(operand)?;
                match operator {
                    UnaryOp::Negate => self.bytecode.push(Opcode::Negate),
                    _ => return Err(self.error("Unary operator not implemented")),
                }
            },
            Expression::Call { function, arguments } => {
                for arg in arguments {
                    self.compile_expression(arg)?;
                }
                
                match &**function {
                    Expression::Identifier(name) => {
                        self.bytecode.push(Opcode::Call(name.clone(), arguments.len()));
                    },
                    _ => return Err(self.error("Only named functions supported currently")),
                }
            },
            Expression::Array { elements } => {
                for element in elements {
                    self.compile_expression(element)?;
                }
                self.bytecode.push(Opcode::MakeArray(elements.len()));
            },
            _ => return Err(self.error("Expression not implemented")),
        }
        Ok(())
    }
    
    fn emit_jump(&mut self, instruction: Opcode) -> usize {
        self.bytecode.push(instruction);
        self.bytecode.len() - 1
    }
    
    fn patch_jump(&mut self, offset: usize) {
        let jump = self.bytecode.len();
        match &mut self.bytecode[offset] {
            Opcode::JumpIfFalse(ref mut val) => *val = jump,
            Opcode::Jump(ref mut val) => *val = jump,
            _ => panic!("Attempted to patch non-jump"),
        }
    }
    
    fn emit_loop(&mut self, loop_start: usize) {
        self.bytecode.push(Opcode::Jump(loop_start));
    }

    #[allow(dead_code)]
    fn serialize_opcode(&self, opcode: &Opcode, bytes: &mut Vec<u8>) {
        // Serialization format needs update for all opcodes
        // 1: LoadConst(f64)
        // 2: LoadString(len, bytes)
        // ...
        // This serialization is critical for runtime compatibility
        // For simplicity, I'm just putting basics here to pass build
        // Real implementation needs robust serialization
        match opcode {
            Opcode::LoadConst(n) => { bytes.push(1); bytes.extend(&n.to_le_bytes()); }
            Opcode::LoadString(s) => { 
                bytes.push(2); 
                bytes.extend(&(s.len() as u32).to_le_bytes()); 
                bytes.extend(s.as_bytes()); 
            }
            Opcode::Output => { bytes.push(8); }
            _ => {} // ToDo: Implement all
        }
    }
    
    fn error(&self, msg: &str) -> ASError {
        ASError::new(ErrorKind::SyntaxError, msg.to_string(), SourceLocation::new(0,0))
    }
}