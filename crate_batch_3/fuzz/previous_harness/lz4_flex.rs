#![no_main]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use lz4_flex::block::compress::compress_prepend_size;
use lz4_flex::block::decompress::decompress_size_prepended;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct CompressedData {
    input: Vec<u8>,
}

fuzz_target!(|data: CompressedData| {
    // Ensure the input is first compressed and then decompressed
    let compressed = compress_prepend_size(&data.input);
    let _ = decompress_size_prepended(&compressed).unwrap_or_default();
});