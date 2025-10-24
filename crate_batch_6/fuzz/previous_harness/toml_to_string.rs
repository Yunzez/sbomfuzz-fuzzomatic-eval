#![no_main]
use libfuzzer_sys::fuzz_target;
use toml::{Value, to_string};
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        if let Ok(value) = toml::from_str::<Value>(s) {
            let _ = to_string(&value);
        }
    }
});