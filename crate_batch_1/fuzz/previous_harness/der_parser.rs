#![no_main]
use libfuzzer_sys::fuzz_target;
use der_parser::der::parse_der;

fuzz_target!(|data: &[u8]| {
    let _ = parse_der(data);
});