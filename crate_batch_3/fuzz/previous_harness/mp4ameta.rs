#![no_main]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use mp4ameta::Tag;
use std::io::{Cursor, Read, Seek};

fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data);
    let _ = Tag::read_from(&mut cursor).ok();
});