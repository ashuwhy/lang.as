// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use crate::compiler::{Compiler, Opcode};
use crate::parser::Parser;
use crate::types::TypeChecker;
use crate::error::{ASError, ErrorKind, SourceLocation};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    None,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Array(elements) => {
                write!(f, "[")?;
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", element)?;
                }
                write!(f, "]")
            }
            Value::None => write!(f, "none"),
        }
    }
}

use crate::resolver::Resolver;

pub struct Runtime {
    compiler: Compiler,
    type_checker: TypeChecker,
    resolver: Resolver,
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    pub debug: bool,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            compiler: Compiler::new(),
            type_checker: TypeChecker::new(),
            resolver: Resolver::new(),
            stack: Vec::new(),
            variables: HashMap::new(),
            debug: false,
        }
    }

    pub fn execute(&mut self, input: &str) -> Result<String, ASError> {
        let ast = Parser::parse(input)?;
        
        // Type check before compilation
        self.type_checker.check(&ast)?;
        
        let bytecode = self.compiler.compile(&ast)?;
        
        self.execute_bytecode(&bytecode)
    }

    fn execute_bytecode(&mut self, bytecode: &[Opcode]) -> Result<String, ASError> {
        let mut pc = 0;
        let mut output = String::new();
        
        while pc < bytecode.len() {
            let opcode = &bytecode[pc];
            if self.debug {
                println!("[DEBUG] PC: {:04} | Op: {:?} | Stack: {:?}", pc, opcode, self.stack);
            }
            pc += 1;
            
            match opcode {
                Opcode::LoadConst(n) => self.stack.push(Value::Number(*n)),
                Opcode::LoadString(s) => self.stack.push(Value::String(s.clone())),
                Opcode::LoadBool(b) => self.stack.push(Value::Boolean(*b)),
                Opcode::LoadVar(name) => {
                    if let Some(val) = self.variables.get(name) {
                        self.stack.push(val.clone());
                    } else {
                        return Err(self.error(&format!("Undefined variable: {}", name)));
                    }
                },
                Opcode::StoreVar(name) => {
                    let val = self.pop()?;
                    self.variables.insert(name.clone(), val);
                },
                Opcode::Output => {
                    let val = self.pop()?;
                    output.push_str(&format!("{}\n", val));
                    // Also print to stdout for interactive feel
                    println!("{}", val);
                },
                Opcode::Input => {
                    // Simple input simulation
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap(); // Handle error properly in real code
                    self.stack.push(Value::String(input.trim().to_string()));
                },
                Opcode::Import(path) => {
                    // 1. Resolve path
                    let resolved_path = self.resolver.resolve(path, None).map_err(|e| self.error(&format!("Import failed: {}", e)))?;
                    
                    // 2. Read file
                    let source = self.resolver.read_file(&resolved_path).map_err(|e| self.error(&format!("Could not read file: {}", e)))?;
                    
                    // 3. Parse
                    let ast = Parser::parse(&source)?;
                    
                    // 4. Type Check (optional for now but good practice)
                    // self.type_checker.check(&ast)?;

                    // 5. Compile
                    // Create a new compiler instance to avoid messing up current bytecode offsets
                    // or append? For simplicity, we execute recursively.
                    let mut compiler = Compiler::new();
                    let bytecode = compiler.compile(&ast)?;
                    
                    // 6. Execute (recursively)
                    // Save PC and bytecode? No, we are in a loop.
                    // Best way is to just call execute_bytecode recursively
                    self.execute_bytecode(&bytecode)?;
                    
                    // Note: Variables defined in imported file will be in self.variables
                    // effectively acting as a global include.
                },
                Opcode::Pop => {
                    self.pop()?;
                },
                
                // Arithmetic
                Opcode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.stack.push(Value::Number(a + b)),
                        (Value::String(a), Value::String(b)) => self.stack.push(Value::String(a + &b)),
                        _ => return Err(self.error("Type mismatch for Add")),
                    }
                },
                Opcode::Subtract => {
                     let b = self.pop_number()?;
                     let a = self.pop_number()?;
                     self.stack.push(Value::Number(a - b));
                },
                Opcode::Multiply => {
                     let b = self.pop_number()?;
                     let a = self.pop_number()?;
                     self.stack.push(Value::Number(a * b));
                },
                Opcode::Divide => {
                     let b = self.pop_number()?;
                     let a = self.pop_number()?;
                     if b == 0.0 { return Err(self.error("Division by zero")); }
                     self.stack.push(Value::Number(a / b));
                },
                
                // Comparison
                Opcode::Eq => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Boolean(a == b));
                },
                Opcode::Gt => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Boolean(a > b));
                },
                Opcode::Lt => {
                    let b = self.pop_number()?;
                    let a = self.pop_number()?;
                    self.stack.push(Value::Boolean(a < b));
                },
                
                // Control Flow
                Opcode::Jump(target) => {
                    pc = *target;
                },
                Opcode::JumpIfFalse(target) => {
                    let val = self.pop()?;
                    let is_true = match val {
                        Value::Boolean(b) => b,
                        Value::Number(n) => n != 0.0,
                        _ => false,
                    };
                    if !is_true {
                        pc = *target;
                    }
                },
                
                _ => return Err(self.error(&format!("Opcode not implemented: {:?}", opcode))),
            }
        }
        
        Ok(output)
    }
    
    fn pop(&mut self) -> Result<Value, ASError> {
        self.stack.pop().ok_or_else(|| self.error("Stack underflow"))
    }
    
    fn pop_number(&mut self) -> Result<f64, ASError> {
        match self.pop()? {
            Value::Number(n) => Ok(n),
            _ => Err(self.error("Expected number")),
        }
    }
    
    fn error(&self, msg: &str) -> ASError {
        ASError::new(ErrorKind::RuntimeError, msg.to_string(), SourceLocation::new(0, 0))
    }
}

pub fn execute(input: &str) -> Result<String, String> {
    let mut runtime = Runtime::new();
    runtime.execute(input).map_err(|e| e.message)
}
