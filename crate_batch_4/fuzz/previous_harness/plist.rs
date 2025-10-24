#![no_main]

use libfuzzer_sys::fuzz_target;
use plist::Value;
use std::io::Cursor;
use arbitrary::Arbitrary;

fuzz_target!(|data: &[u8]| {
    let cursor = Cursor::new(data);
    let _ = Value::from_reader(cursor);
});