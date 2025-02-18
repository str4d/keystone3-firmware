#![no_std]

extern crate alloc;

use alloc::string::String;
use cstr_core::CString;
use cty::c_char;

pub mod binding;
pub mod macros;

pub fn convert_c_char(s: String) -> *mut c_char {
    CString::new(s.clone()).unwrap().into_raw()
}
