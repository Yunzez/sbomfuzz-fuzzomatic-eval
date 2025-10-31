#![no_main]

extern crate libfuzzer_sys;

use crate_batch_4::run_11;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    run_11(data);
});
