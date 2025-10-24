#![no_main]

use libfuzzer_sys::fuzz_target;
use ubyte::ByteUnit;
use std::str::FromStr;

fuzz_target!(|data: &str| {
    // Fuzzing harness for ByteUnit::from_str
    let _ = ByteUnit::from_str(data);
});