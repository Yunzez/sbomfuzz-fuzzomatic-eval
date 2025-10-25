#![no_main]

extern crate libfuzzer_sys;
extern crate bcrypt;

use libfuzzer_sys::fuzz_target;
use bcrypt::verify;
use std::ffi::CStr;

fuzz_target!(|data: &[u8]| {
    if let Ok(c_str) = CStr::from_bytes_with_nul(data) {
        if let Ok(password) = c_str.to_str() {
            // Example bcrypt hash, typically sourced from your test corpus
            let hash = "$2y$12$XZ6J8vZc6Q1jz2X1Z5Q5eOe5eOe5eOe5eOe5eOe5eOe5eOe5eOe";
            let _ = verify(password, hash);
        }
    }
});