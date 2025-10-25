#![no_main]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
extern crate der;
use der::Decodable;
use der::Decoder;
// this is the fix: use der::Decodable;
fuzz_target!(|data: &[u8]| {
    if let Ok(mut decoder) = Decoder::new(data) {
        let _ = der::asn1::Any::decode(&mut decoder).ok();
    }
});