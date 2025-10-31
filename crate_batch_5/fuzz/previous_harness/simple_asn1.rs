#![no_main]

use libfuzzer_sys::fuzz_target;
use simple_asn1::from_der;

fuzz_target!(|data: &[u8]| {
    let _ = from_der(data);
});
