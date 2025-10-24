#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Cursor;
use ini::Ini;

fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data);
    // Attempt to read from the cursor, ignoring results, as we're only checking for crashes
    let _ = Ini::read_from(&mut cursor);
});