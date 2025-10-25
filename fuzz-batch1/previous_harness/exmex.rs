#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use exmex::parse_with_default_ops;

fuzz_target!(|data: &str| {
    let _ = parse_with_default_ops::<f64>(data);
});