#![no_main]


extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;

// extern crate yaxpeax_x86;

use yaxpeax_x86::long_mode::InstDecoder;

/// Fuzzing harness for `InstDecoder::default`
fuzz_target!(|data: &[u8]| {
    let decoder = InstDecoder::default();
    let _ = decoder.decode_slice(data);
});
