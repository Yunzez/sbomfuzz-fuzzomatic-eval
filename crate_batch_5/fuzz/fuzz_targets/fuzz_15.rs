#![no_main]

extern crate crate_batch_5;
extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    crate_batch_5::run_15(data);
});
