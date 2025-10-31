#![no_main]
extern crate libfuzzer_sys;
extern crate pcapng;

use libfuzzer_sys::fuzz_target;
use pcapng::block::parse_block;

fuzz_target!(|data: &[u8]| {
    let _ = parse_block(data);
});