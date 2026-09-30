// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::runtime;

/// Execute AS Lang code and return the output as a C string.
/// The caller is responsible for freeing the returned string using `as_free_string`.
#[no_mangle]
pub extern "C" fn as_execute(code: *const c_char) -> *mut c_char {
    if code.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(code) };
    let input = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => return CString::new("Error: Invalid UTF-8").unwrap().into_raw(),
    };

    let result = runtime::execute(input);
    
    let output = match result {
        Ok(s) => s,
        Err(e) => format!("Error: {}", e),
    };

    CString::new(output).unwrap().into_raw()
}

/// Free a string returned by `as_execute`.
#[no_mangle]
pub extern "C" fn as_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
