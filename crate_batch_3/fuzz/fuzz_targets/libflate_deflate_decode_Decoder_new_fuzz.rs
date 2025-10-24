#![no_main]
use libfuzzer_sys::fuzz_target;
use libflate::deflate::Decoder;
use arbitrary::Arbitrary;
use std::io;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let reader = &input.data[..];
    let mut decoder = Decoder::new(reader);
    let _ = io::copy(&mut decoder, &mut io::sink());
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: libflate 
//  - Crate Link: https://github.com/sile/libflate 
//  - Crate Version: 1.1.1 
//  - From Crate: crate_batch_3 
//  - From Crate Link: unknown_website 
//  - Module Path: libflate::deflate::decode::(Struct)Decoder 
//  - Use Statement: None 
//  - Function Name: new 
//  - Function Usage: fn run_1() {
//     // ! this input crash the program
//     // let input = b"\x04\x04\x04\x05:\x1az*\xfc\x06\x01\x90\x01\x06\x01";
// 
//     let input = b"\x04\x04\x04\x04:\x1az*\xfc\x06\x01\x90\x01\x06\x01";
//     let mut decoder = libflate::deflate::Decoder::new(&input[..]);
// 
//     let result = io::copy(&mut decoder, &mut io::sink());
// } 
//  - Parameters: initial function signature:No type_fields: fn(R) -> Decoder<R>

