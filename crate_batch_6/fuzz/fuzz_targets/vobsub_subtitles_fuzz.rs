#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use vobsub::subtitles;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a [u8],
}

fuzz_target!(|input: FuzzInput| {
    // Using the fuzzed data directly in the subtitles function
    for _ in subtitles(input.data) {
        // Iterate over subtitles and ignore for fuzzing purposes
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: vobsub 
//  - Crate Link: None 
//  - Crate Version: 0.1.1 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: vobsub::sub 
//  - Use Statement: None 
//  - Function Name: subtitles 
//  - Function Usage: fn run_12() {
//     // ? line 219 - 223, same entry function
//     println!("run 12");
//     let data = b"";
//     for _ in vobsub::subtitles(data) {
//         // Just parse and ignore.
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&[u8]) -> Subtitles<'_>,
//   type_fields: [vobsub::Subtitles] 
// }
// Struct construction metadata: {
//   type_name: fn(&[u8]) -> Subtitles<'_>,
//   type_fields: [vobsub::Subtitles] 
// }
// Struct construction metadata: {
//   type_name: vobsub::Subtitles,
//   type_fields: [vobsub::PesPackets] 
// }
// Struct construction metadata: {
//   type_name: vobsub::PesPackets,
//   type_fields: [&[u8]] 
// }

