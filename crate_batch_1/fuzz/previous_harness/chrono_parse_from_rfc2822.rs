#![no_main]

extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;
use chrono_16::DateTime;

fuzz_target!(|data: &str| {
    // Using the parse_from_rfc2822 function with fuzzed input
    let _ = DateTime::parse_from_rfc2822(data);
});