#![no_main]

extern crate libfuzzer_sys;

use crate_batch_3::main;
use libfuzzer_sys::fuzz_target; // Import the main function from the crate_batch_3 module

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(_input) = std::str::from_utf8(data) {
        main(); // Call `main` without arguments
    }
});
