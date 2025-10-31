#![no_main]

extern crate crate_batch_1;
extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    crate_batch_1::run_12(data);
});
