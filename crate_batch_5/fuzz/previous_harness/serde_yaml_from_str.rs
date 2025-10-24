#![no_main]
use libfuzzer_sys::fuzz_target;
use serde_yaml::from_str;

fuzz_target!(|data: &str| {
    let _: Result<serde_yaml::Value, _> = from_str(data);
});