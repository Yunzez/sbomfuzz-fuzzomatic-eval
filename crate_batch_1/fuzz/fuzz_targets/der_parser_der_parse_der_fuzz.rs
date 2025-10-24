#![no_main]
use libfuzzer_sys::fuzz_target;
use der_parser::der::parse_der;
use arbitrary::Arbitrary;

fuzz_target!(|data: &[u8]| {
    let _ = parse_der(data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: der_parser 
//  - Crate Link: https://github.com/rusticata/der-parser 
//  - Crate Version: 0.3.1 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: der_parser::der 
//  - Use Statement: None 
//  - Function Name: parse_der 
//  - Function Usage: fn run_13() {
//     let data =
//         b"\x03\x00\x00kk\x00\x00\x00\x00\x00\x00\x00.\x00\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff;\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\xff\x0a\xff";
//     let _ = der_parser::parse_der(data);
// } 
//  - Parameters: initial function signature:No type_fields: fn(&[u8]) -> IResult<&[u8], DerObject<'_>, u32>

