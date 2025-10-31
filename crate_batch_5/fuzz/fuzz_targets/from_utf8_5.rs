#![no_main]

extern crate libfuzzer_sys;

use crate_batch_5::run_18;
use libfuzzer_sys::fuzz_target; // Adjusted import to directly use run_18

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(input) = std::str::from_utf8(data) {
        run_18(input);
    }
});
