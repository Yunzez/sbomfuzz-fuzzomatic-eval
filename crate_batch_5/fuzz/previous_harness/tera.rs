#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str;
use tera_190::{Context, Tera};

fuzz_target!(|data: &[u8]| {
    if let Ok(input_str) = str::from_utf8(data) {
        let context = Context::new();
        let _ = Tera::one_off(input_str, &context, true);
    }
});