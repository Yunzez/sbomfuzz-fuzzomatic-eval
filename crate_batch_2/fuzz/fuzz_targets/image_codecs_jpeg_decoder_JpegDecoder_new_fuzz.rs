#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::io::Cursor;
use image::codecs::jpeg::JpegDecoder;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let _ = JpegDecoder::new(Cursor::new(input.data));
});