#![no_main]

extern crate libfuzzer_sys;

use crate_batch_1::run_6;
use libfuzzer_sys::fuzz_target; // Adjusted import to directly use run_6

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(input) = std::str::from_utf8(data) {
        run_6(input); // Directly call run_6
    }
});
