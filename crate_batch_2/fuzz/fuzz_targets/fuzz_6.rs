#![no_main]

extern crate libfuzzer_sys;

use crate_batch_2::run_6;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    run_6(data);
});
