#![no_main]

use libfuzzer_sys::fuzz_target;
use std::io::Cursor;
use zip::ZipArchive;

fuzz_target!(|data: &[u8]| {
    let mut reader = Cursor::new(data);
    reader.set_position(0);
    let _ = ZipArchive::new(reader);
});
