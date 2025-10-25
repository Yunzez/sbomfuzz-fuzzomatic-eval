#![no_main]


use libfuzzer_sys::fuzz_target;
use bson_10::decode_document;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let mut reader = Cursor::new(data);
    let result = decode_document(&mut reader).is_err();
    assert!(result);
});