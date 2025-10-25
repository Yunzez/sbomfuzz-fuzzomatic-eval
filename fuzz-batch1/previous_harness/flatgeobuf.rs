#![no_main]
extern crate libfuzzer_sys;
use std::io::Cursor;
use std::io::BufReader;
use flatgeobuf::*;
use libfuzzer_sys::fuzz_target;
fuzz_target!(|data: &[u8]| {
    let mut buf_reader = BufReader::new(Cursor::new(data));
    if let Ok(mut fgb) = FgbReader::open(&mut buf_reader) {
        let _ = fgb.header();
    }
});