#![no_main]

extern crate libfuzzer_sys;

use crate_batch_1::run_17;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    run_17(data);
});
