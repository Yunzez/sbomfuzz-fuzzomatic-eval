#![no_main]
use libfuzzer_sys::fuzz_target;
use serde_yaml::from_slice;

fuzz_target!(|data: &[u8]| {
    let _: Result<serde_yaml::Value, _> = from_slice(data);
});
