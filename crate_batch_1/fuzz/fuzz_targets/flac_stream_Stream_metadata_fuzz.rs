#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use flac::stream::Stream;
use flac::ByteStream;

fuzz_target!(|data: Vec<u8>| {
    if let Ok(mut stream) = Stream::<ByteStream>::from_buffer(&data) {
        let _ = stream.metadata();
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: flac 
//  - Crate Link: None 
//  - Crate Version: 0.5.0 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: flac::stream::(Struct)Stream 
//  - Use Statement: None 
//  - Function Name: info 
//  - Function Usage: fn run_16() {
//     // ! flac
//     let s = Stream::<ByteStream>::from_buffer(b"fLaC\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00H\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\\");
//     if let Ok(mut stream) = s {
//         let _ = stream.info();
//         let _ = stream.metadata();
//         let mut iter = stream.iter::<i8>();
//         while iter.next().is_some() { }
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&Stream<P>) -> StreamInfo,
//   type_fields: [P, flac::StreamInfo] 
// }
// Struct construction metadata: {
//   type_name: fn(&Stream<P>) -> StreamInfo,
//   type_fields: [P, flac::StreamInfo] 
// }
// Struct construction metadata: {
//   type_name: flac::StreamInfo,
//   type_fields: [u16, u16, u32, u32, u32, u8, u8, u64, [u8; 16]] 
// }

