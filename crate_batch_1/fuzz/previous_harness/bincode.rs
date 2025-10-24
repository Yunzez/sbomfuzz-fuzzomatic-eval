#![no_main]

use libfuzzer_sys::fuzz_target;
use bincode_6::{config::Configuration, decode_from_slice};

fuzz_target!(|data: &[u8]| {
    // Attempt to decode using bincode's decode_from_slice function
    // let config = Configuration::standard();
    let _: Result<(u32, _), _> = decode_from_slice(data, config);
});