#![no_main]

use alloy_json_abi::JsonAbi;
use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let lines = s.lines();
        // Execute the function with the converted iterator
        let _ = JsonAbi::parse(lines);
    }
});