#![no_main]

extern crate libfuzzer_sys;

use crate_batch_3::run_13;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    run_13(data);
});
