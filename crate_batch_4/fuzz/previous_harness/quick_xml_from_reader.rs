#![no_main]
use std::io::Cursor;
use libfuzzer_sys::fuzz_target;
use quick_xml::reader::Reader;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return; // Ensure the input has enough length to avoid unnecessary crashes
    }
    let cursor = Cursor::new(data);
    let mut reader = Reader::from_reader(cursor);
    let mut buf = Vec::new();
    let _ = reader.read_event(&mut buf);
});