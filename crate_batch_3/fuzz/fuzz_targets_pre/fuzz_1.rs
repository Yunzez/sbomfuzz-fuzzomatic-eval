#![no_main]

extern crate crate_batch_3;
extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    crate_batch_3::run_1(data);
});
