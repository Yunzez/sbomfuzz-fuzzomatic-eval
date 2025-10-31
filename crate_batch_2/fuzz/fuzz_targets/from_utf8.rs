#![no_main]

extern crate crate_batch_2;
extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(input) = std::str::from_utf8(data) {
        crate_batch_2::run_update(input);
    }
});
