#![no_main]
use libfuzzer_sys::fuzz_target;
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct CompressToVecInput<'a> {
    input: &'a [u8],
    level: u8,
}

fuzz_target!(|data: CompressToVecInput| {
    let compressed_data = compress_to_vec(data.input, data.level);
    let result =  decompress_to_vec(&compressed_data);
});