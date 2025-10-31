#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::Cursor;
use ws::Frame;

fuzz_target!(|data: Vec<u8>| {
    let mut cursor = Cursor::new(data);
    let _ = Frame::parse(&mut cursor);
});