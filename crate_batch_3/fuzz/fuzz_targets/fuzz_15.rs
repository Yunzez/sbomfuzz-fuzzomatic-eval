#![no_main]

extern crate libfuzzer_sys;

use crate_batch_3::run_15;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    // fuzzed code goes here
    run_15(data);
});
