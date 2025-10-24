#![no_main]
use libfuzzer_sys::fuzz_target;
use rustc_demangle::demangle;

fuzz_target!(|data: &str| {
    let _ = demangle(data);
});