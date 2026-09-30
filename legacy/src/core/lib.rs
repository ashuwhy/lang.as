// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::wrap_pyfunction;

pub mod compiler;
pub mod parser;
pub mod runtime;
pub mod error;
pub mod lexer;
pub mod types;
pub mod resolver;
pub mod ffi;

pub use compiler::*;
pub use parser::*;
pub use runtime::*;

pub const VERSION: &str = "0.1.0";
pub const AUTHOR: &str = "Ashutosh Sharma <ashutoshsharmawhy@gmail.com>";
pub const COPYRIGHT: &str = "© 2026 Ashutosh Sharma";

#[cfg(feature = "python")]
#[pyfunction]
fn run_code(source: String) -> PyResult<String> {
    let mut runtime = runtime::Runtime::new();
    match runtime.execute(&source) {
        Ok(output) => Ok(output),
        Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("{}", e))),
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("VERSION", VERSION)?;
    m.add_function(wrap_pyfunction!(run_code, m)?)?;
    Ok(())
} 