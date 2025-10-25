#![no_main]

extern crate libfuzzer_sys;

use crate_batch_5::run_14;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    run_14(data);
});
