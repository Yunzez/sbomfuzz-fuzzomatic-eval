#![no_main]
extern crate libfuzzer_sys;
extern crate gif;

use libfuzzer_sys::fuzz_target;
use gif::DecodeOptions;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let mut options = DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let cursor = Cursor::new(data);
    if let Ok(mut decoder) = options.read_info(data) {
        while let Ok(Some(_frame)) = decoder.read_next_frame() {}
    }
});