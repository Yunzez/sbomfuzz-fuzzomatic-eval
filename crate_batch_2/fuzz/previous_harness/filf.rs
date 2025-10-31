#![no_main]

use libfuzzer_sys::fuzz_target;
use flif::Flif;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let _ = Flif::decode(Cursor::new(data)).map(|img| img.get_raw_pixels());
});