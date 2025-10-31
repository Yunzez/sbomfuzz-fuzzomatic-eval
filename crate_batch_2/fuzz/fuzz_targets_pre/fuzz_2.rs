#![no_main]

extern crate crate_batch_2;
extern crate libfuzzer_sys;

use crate_batch_2::run_2;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    run_2(data);
});
