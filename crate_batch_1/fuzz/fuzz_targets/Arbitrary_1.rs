#![no_main]

extern crate libfuzzer_sys;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
pub struct Arguments {
    pub arg1: u8,
    pub arg2: u8,
}

fuzz_target!(|arguments: Arguments| {
    // Convert u8 values to &[u8] slices
    let arg1_slice = &[arguments.arg1];
    let arg2_slice = &[arguments.arg2];

    // Call the function with the correct argument types
    crate_batch_1::run_5(arg1_slice, arg2_slice);
});
