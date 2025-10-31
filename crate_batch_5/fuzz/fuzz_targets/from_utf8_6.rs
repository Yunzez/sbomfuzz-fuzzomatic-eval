#![no_main]

extern crate libfuzzer_sys;

use crate_batch_5::run_20;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(input) = std::str::from_utf8(data) {
        run_20(input);
    }
});
