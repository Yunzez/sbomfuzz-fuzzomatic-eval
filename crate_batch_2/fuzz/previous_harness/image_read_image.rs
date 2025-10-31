#![no_main]
use libfuzzer_sys::fuzz_target;
use image::codecs::jpeg::JpegDecoder;
use jpeg_decoder_64::Decoder;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let mut decoder = Decoder::new(data);
    let _pixels = decoder.decode();
    let _metadata = decoder.info();
});