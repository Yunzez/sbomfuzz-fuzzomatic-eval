#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use image::codecs::webp::WebPDecoder;
use std::io::Cursor;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let cursor = Cursor::new(input.data);
    let _ = WebPDecoder::new(cursor);
});