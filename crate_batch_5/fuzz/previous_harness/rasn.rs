#![no_main]

extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use rasn::der;
use rasn::types::Open;
use rasn::Decode;

fuzz_target!(|data: &[u8]| {
    let _: Result<Open, _> = der::decode(data);
});