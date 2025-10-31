#![no_main]
use libfuzzer_sys::fuzz_target;
use snap::Decoder;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let mut decoder = Decoder::new();

    let _ = decoder.decompress_vec(&input.data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: snap 
//  - Crate Link: https://github.com/BurntSushi/rust-snappy 
//  - Crate Version: 0.2.2 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: snap::decompress::(Struct)Decoder 
//  - Use Statement: None 
//  - Function Name: decompress_vec 
//  - Function Usage: fn run_9() {
//     let crashing_input: &[u8] = &[
//         0x82, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00, 0x00,
//     ];
//     let mut decoder = Decoder::new();
// 
//     match decoder.decompress_vec(crashing_input) {
//         Ok(_) => println!("Decompression succeeded."),
//         Err(e) => eprintln!("Decompression failed: {:?}", e),
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut Decoder, &[u8]) -> Result<Vec<u8, Global>, Error>,
//   type_fields: [snap::Decoder, alloc::Global, snap::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut Decoder, &[u8]) -> Result<Vec<u8, Global>, Error>,
//   type_fields: [snap::Decoder, alloc::Global, snap::Error] 
// }
// Struct construction metadata: {
//   type_name: snap::Decoder,
//   type_fields: [()] 
// }
// Struct construction metadata: {
//   type_name: snap::Error,
//   type_fields: [TooBig, BufferTooSmall, Empty, Header, HeaderMismatch, Literal, CopyRead, CopyWrite, Offset, StreamHeader, StreamHeaderMismatch, UnsupportedChunkType, UnsupportedChunkLength, Checksum] 
// }

