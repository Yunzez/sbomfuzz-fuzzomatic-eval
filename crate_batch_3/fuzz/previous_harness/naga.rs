#![no_main]
extern crate libfuzzer_sys;
extern crate naga;

use libfuzzer_sys::fuzz_target;
use naga::front::wgsl::Parser;

fuzz_target!(|data: &[u8]| {
    if let Ok(data_str) = std::str::from_utf8(data) {
        let mut parser = Parser::new();
        let _ = parser.parse(data_str);
    }
});