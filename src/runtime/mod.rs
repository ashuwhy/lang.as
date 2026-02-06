// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use crate::compiler::Compiler;
use crate::parser::Parser;
use std::collections::HashMap;

#[derive(Clone)]
enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    None,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
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

pub struct Runtime {
    parser: Parser,
    compiler: Compiler,
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    functions: HashMap<String, usize>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            parser: Parser::new(),
            compiler: Compiler::new(),
            stack: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn execute(&mut self, input: &str) -> Result<String, String> {
        let ast = self.parser.parse(input)?;
        let bytecode = self.compiler.compile(&ast)?;
        
        self.execute_bytecode(&bytecode)
    }

    fn execute_bytecode(&mut self, bytecode: &[u8]) -> Result<String, String> {
        let mut pc = 0;
        let mut output = String::new();
        
        while pc < bytecode.len() {
            match bytecode[pc] {
                1 => { // LoadConst
                    pc += 1;
                    let n = f64::from_le_bytes(bytecode[pc..pc+8].try_into().unwrap());
                    pc += 8;
                    self.stack.push(Value::Number(n));
                }
                2 => { // LoadString
                    pc += 1;
                    let len = u32::from_le_bytes(bytecode[pc..pc+4].try_into().unwrap()) as usize;
                    pc += 4;
                    let s = String::from_utf8(bytecode[pc..pc+len].to_vec())
                        .map_err(|e| e.to_string())?;
                    pc += len;
                    self.stack.push(Value::String(s));
                }
                3 => { // LoadVar
                    pc += 1;
                    let idx = u32::from_le_bytes(bytecode[pc..pc+4].try_into().unwrap()) as usize;
                    pc += 4;
                    if let Some(var) = self.variables.get(&idx.to_string()) {
                        self.stack.push(var.clone());
                    } else {
                        return Err(format!("Undefined variable at index {}", idx));
                    }
                }
                4 => { // StoreVar
                    pc += 1;
                    let idx = u32::from_le_bytes(bytecode[pc..pc+4].try_into().unwrap()) as usize;
                    pc += 4;
                    if let Some(value) = self.stack.pop() {
                        self.variables.insert(idx.to_string(), value);
                    } else {
                        return Err("Stack underflow".to_string());
                    }
                }
                5 => { // Call
                    pc += 1;
                    let func_pos = u32::from_le_bytes(bytecode[pc..pc+4].try_into().unwrap()) as usize;
                    pc += 4;
                    let argc = u32::from_le_bytes(bytecode[pc..pc+4].try_into().unwrap()) as usize;
                    pc += 4;
                    
                    // Save current position
                    self.functions.insert("return".to_string(), pc);
                    
                    // Jump to function
                    pc = func_pos;
                    
                    // Setup function arguments
                    let mut args = Vec::new();
                    for _ in 0..argc {
                        if let Some(arg) = self.stack.pop() {
                            args.push(arg);
                        } else {
                            return Err("Stack underflow in function call".to_string());
                        }
                    }
                    args.reverse();
                    
                    // Push arguments to variables
                    for (i, arg) in args.into_iter().enumerate() {
                        self.variables.insert(i.to_string(), arg);
                    }
                }
                6 => { // MakeArray
                    pc += 1;
                    let size = u32::from_le_bytes(bytecode[pc..pc+4].try_into().unwrap()) as usize;
                    pc += 4;
                    
                    let mut elements = Vec::new();
                    for _ in 0..size {
                        if let Some(element) = self.stack.pop() {
                            elements.push(element);
                        } else {
                            return Err("Stack underflow in array creation".to_string());
                        }
                    }
                    elements.reverse();
                    self.stack.push(Value::Array(elements));
                }
                7 => { // Return
                    if let Some(return_pos) = self.functions.remove("return") {
                        pc = return_pos;
                    } else {
                        break;
                    }
                }
                8 => { // Output
                    pc += 1;
                    if let Some(value) = self.stack.pop() {
                        output.push_str(&value.to_string());
                        output.push('\n');
                    } else {
                        return Err("Stack underflow in output".to_string());
                    }
                }
                _ => return Err(format!("Invalid opcode: {}", bytecode[pc])),
            }
        }
        
        Ok(output)
    }
}

pub fn execute(input: &str) -> Result<String, String> {
    let mut runtime = Runtime::new();
    runtime.execute(input)
} 