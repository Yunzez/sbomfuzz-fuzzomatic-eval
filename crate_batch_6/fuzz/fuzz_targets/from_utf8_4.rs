#![no_main]

extern crate libfuzzer_sys;

use crate_batch_6;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(_input) = std::str::from_utf8(data) {
        crate_batch_6::run_4();
    }
});
