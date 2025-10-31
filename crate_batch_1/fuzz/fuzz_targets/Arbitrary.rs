#![no_main]

extern crate libfuzzer_sys;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

// Assuming the correct path to the function is directly under crate_batch_1
use crate_batch_1::run_3;

#[derive(Arbitrary, Debug)]
pub struct Arguments {
    pub arg1: u8,
    pub arg2: u8,
}

fuzz_target!(|arguments: Arguments| {
    // Convert u8 to String and then to &str
    let arg1_str = arguments.arg1.to_string();
    let arg2_str = arguments.arg2.to_string();

    // Call run_3 with &str arguments
    run_3(&arg1_str, &arg2_str);
});
