#![no_main]
use libfuzzer_sys::fuzz_target;
use snap::Decoder;

fuzz_target!(|data: &[u8]| {
    let mut decoder = Decoder::new();

    let _ = decoder.decompress_vec(data);
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
//  - Use Statement: use snap::Decoder; 
//  - Function Name: new 
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
//   type_name: fn() -> Decoder,
//   type_fields: [snap::Decoder] 
// }
// Struct construction metadata: {
//   type_name: fn() -> Decoder,
//   type_fields: [snap::Decoder] 
// }
// Struct construction metadata: {
//   type_name: snap::Decoder,
//   type_fields: [()] 
// }

