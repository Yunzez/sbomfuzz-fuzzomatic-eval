#![no_main]
use libfuzzer_sys::fuzz_target;
use image::codecs::webp::WebPDecoder;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let _ = WebPDecoder::new(Cursor::new(data));
});