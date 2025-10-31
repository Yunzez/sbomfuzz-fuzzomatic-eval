#![no_main]
extern crate libfuzzer_sys;
extern crate ron;
extern crate arbitrary;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use ron::de::from_str;

fuzz_target!(|data: &str| {
    // Attempt to parse the input string using from_str
    let _ = from_str::<ron::Value>(data);
});