#![no_main]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use nom::{named, take};

// Define the parser using the macro.
#[macro_use]
extern crate nom;

named!(parser02<&[u8], ()>,
    do_parse!(
        hdr: take!(1) >>
        data: take!(18446744073709551615) >> // Large value for fuzzing
        ( () )
    )
);

// Entry point for fuzzing
fuzz_target!(|data: &[u8]| {
    // Use the parsers defined above with the fuzzing data
    let _ = parser02(data); 
});
