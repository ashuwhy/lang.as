// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

pub mod compiler;
pub mod parser;
pub mod runtime;
pub mod error;
pub mod lexer;

pub use compiler::*;
pub use parser::*;
pub use runtime::*;

pub const VERSION: &str = "0.1.0";
pub const AUTHOR: &str = "Ashutosh Sharma <ashutoshsharmawhy@gmail.com>";
pub const COPYRIGHT: &str = "© 2026 Ashutosh Sharma"; 