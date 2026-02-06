// Copyright (c) 2025 Ashutosh Sharma. All rights reserved.

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/// WASM runtime wrapper for AS Lang
#[wasm_bindgen]
pub struct WasmRuntime {
    inner: aslang::runtime::Runtime,
}

#[wasm_bindgen]
impl WasmRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmRuntime {
        WasmRuntime {
            inner: aslang::runtime::Runtime::new(),
        }
    }

    /// Execute AS Lang code and return the output as a string
    pub fn execute(&mut self, code: &str) -> Result<String, JsValue> {
        self.inner.execute(code)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// Compile AS Lang code and return compilation result as JSON
#[wasm_bindgen]
pub fn compile(source: &str) -> Result<JsValue, JsValue> {
    let ast = aslang::parser::Parser::parse(source)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let mut compiler = aslang::compiler::Compiler::new();
    let bytecode = compiler.compile(&ast)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let result = CompilationResult {
        bytecode_len: bytecode.len(),
        success: true,
        message: "Compiled successfully".to_string(),
    };
    
    Ok(serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .into())
}

/// Execute AS Lang code directly (convenience function)
#[wasm_bindgen]
pub fn run(code: &str) -> Result<String, JsValue> {
    aslang::runtime::execute(code)
        .map_err(|e| JsValue::from_str(&e))
}

#[derive(Serialize, Deserialize)]
struct CompilationResult {
    bytecode_len: usize,
    success: bool,
    message: String,
}

/// Initialize WASM module (sets up panic hook for better error messages)
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages in development
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}