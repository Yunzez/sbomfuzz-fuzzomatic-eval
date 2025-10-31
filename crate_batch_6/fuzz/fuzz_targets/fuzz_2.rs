#![no_main]

extern crate libfuzzer_sys;

use crate_batch_6::run_2;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    run_2(data);
});
