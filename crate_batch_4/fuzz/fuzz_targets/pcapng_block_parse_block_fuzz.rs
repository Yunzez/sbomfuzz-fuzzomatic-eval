#![no_main]
use libfuzzer_sys::fuzz_target;
use pcapng::block::parse_block;

fuzz_target!(|data: &[u8]| {
    let _ = parse_block(data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: pcapng 
//  - Crate Link: None 
//  - Crate Version: 1.0.0 
//  - From Crate: crate_batch_4 
//  - From Crate Link: unknown_website 
//  - Module Path: pcapng::block 
//  - Use Statement: None 
//  - Function Name: parse_block 
//  - Function Usage: fn run_2() {
//     // ! crashing output:
//     // let data = b"h;\x00\x00\x00\x00\x00\x00\x00\x00\x07/\x8a";
// 
//     let data = b"\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19";
//     let _ = pcapng::block::parse_block(data);
// } 
//  - Parameters: initial function signature:No type_fields: fn(&[u8]) -> IResult<&[u8], RawBlock<'_>, u32>

