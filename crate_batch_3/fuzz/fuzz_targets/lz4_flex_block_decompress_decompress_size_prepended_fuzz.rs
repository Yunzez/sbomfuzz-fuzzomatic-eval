#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use lz4_flex::block::compress::compress_prepend_size;
use lz4_flex::block::decompress::decompress_size_prepended;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let compressed = compress_prepend_size(&input.data);
    let _ = decompress_size_prepended(&compressed).ok();
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
//  - Module Path: lz4_flex::block::decompress 
//  - Use Statement: use lz4_flex::block::decompress::decompress_size_prepended; 
//  - Function Name: decompress_size_prepended 
//  - Function Usage: fn run_4() {
//     // ? line 80, 81 are the same function
//     let data = b"Some data to compress and decompress using lz4_flex";
//     let compressed = compress_prepend_size(data);
//     let decompressed = decompress_size_prepended(&compressed).unwrap();
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&[u8]) -> Result<Vec<u8, Global>, Error>,
//   type_fields: [alloc::Global, lz4_flex::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&[u8]) -> Result<Vec<u8, Global>, Error>,
//   type_fields: [alloc::Global, lz4_flex::Error] 
// }
// Struct construction metadata: {
//   type_name: lz4_flex::Error,
//   type_fields: [OutputTooSmall, LiteralOutOfBounds, ExpectedAnotherByte, OffsetOutOfBounds] 
// }

