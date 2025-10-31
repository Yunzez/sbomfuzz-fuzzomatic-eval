#![no_main]

extern crate crate_batch_5;
extern crate libfuzzer_sys; // Add this line to import the external crate

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(input) = std::str::from_utf8(data) {
        crate_batch_5::run_13(input);
    }
});
