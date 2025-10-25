#![no_main]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use csscolorparser::parse;

fuzz_target!(|data: &str| {

        let _ = parse(data);
});