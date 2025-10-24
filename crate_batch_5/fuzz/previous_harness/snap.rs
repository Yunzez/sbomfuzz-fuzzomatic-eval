#![no_main]

use libfuzzer_sys::fuzz_target;
use snap::Decoder;

fuzz_target!(|data: &[u8]| {
    let mut decoder = Decoder::new();
    let _ = decoder.decompress_vec(data);
});