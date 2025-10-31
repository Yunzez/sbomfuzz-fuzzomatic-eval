#![no_main]
extern crate libfuzzer_sys;
extern crate serde_hjson;

use libfuzzer_sys::fuzz_target;
use serde_hjson::Value;
use serde_hjson::Map;

fuzz_target!(|data: &[u8]| {
    let _: Result<Map<String, Value>, _> = serde_hjson::from_slice(data);
});