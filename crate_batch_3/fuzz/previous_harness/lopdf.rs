#![no_main]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use lopdf::Document;

fuzz_target!(|data: &[u8]| {
    let _ = Document::load_mem(data);
});