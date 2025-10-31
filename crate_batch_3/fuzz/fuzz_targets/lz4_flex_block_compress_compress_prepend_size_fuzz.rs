#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use lz4_flex::block::compress::compress_prepend_size;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let _ = compress_prepend_size(&input.data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: lz4_flex 
//  - Crate Link: unknown_website 
//  - Crate Version: 0.2.0 
//  - From Crate: crate_batch_3 
//  - From Crate Link: unknown_website 
//  - Module Path: lz4_flex::block::compress 
//  - Use Statement: use lz4_flex::block::compress::compress_prepend_size; 
//  - Function Name: compress_prepend_size 
//  - Function Usage: fn run_4() {
//     // ? line 80, 81 are the same function
//     let data = b"Some data to compress and decompress using lz4_flex";
//     let compressed = compress_prepend_size(data);
//     let decompressed = decompress_size_prepended(&compressed).unwrap();
// } 
//  - Parameters: initial function signature:No type_fields: fn(&[u8]) -> Vec<u8, Global>

