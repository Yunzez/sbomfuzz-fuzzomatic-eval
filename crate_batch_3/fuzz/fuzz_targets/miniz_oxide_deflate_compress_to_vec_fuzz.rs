#![no_main]
use libfuzzer_sys::fuzz_target;
use miniz_oxide::deflate::compress_to_vec;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a [u8],
    level: u8,
}

fuzz_target!(|fuzz_data: FuzzInput| {
    // Compress the input data using a provided compression level.
    let _compressed_data = compress_to_vec(fuzz_data.data, fuzz_data.level);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: miniz_oxide 
//  - Crate Link: unknown_website 
//  - Crate Version: 0.1.2 
//  - From Crate: crate_batch_3 
//  - From Crate Link: unknown_website 
//  - Module Path: miniz_oxide::deflate 
//  - Use Statement: use miniz_oxide::deflate::compress_to_vec; 
//  - Function Name: compress_to_vec 
//  - Function Usage: fn run_8() {
//     // ? line 94 - 95, doesn't work
//     return;
//     // Manually create DecompressorOxide (forces initialization)
//     let mut decompressor = DecompressorOxide::new();
// 
//     // Compress data using raw deflate
//     let compressed_data = compress_to_vec(b"Hello, world!", 6);
// 
//     let result = panic::catch_unwind(|| { decompress_to_vec(&compressed_data) });
// 
//     match result {
//         Ok(Ok(_)) => println!("Decompression successful!"),
//         Ok(Err(_)) => eprintln!("Decompression failed gracefully."),
//         Err(_) => eprintln!("Panic occurred during decompression!"),
//     }
// } 
//  - Parameters: initial function signature:No type_fields: fn(&[u8], u8) -> Vec<u8, Global>

