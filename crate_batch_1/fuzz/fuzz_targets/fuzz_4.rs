#![no_main]

extern crate libfuzzer_sys;

use crate_batch_1::run_4;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    run_4(data);
});
