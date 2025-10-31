#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use goblin::archive::Index;
use goblin::error::Error;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    buffer: &'a [u8],
}

fuzz_target!(|data: FuzzInput| {
    match Index::parse_sysv_index(data.buffer) {
        Ok(index) => {
            println!("Parsed index: {:?}", index);
        }
        Err(err) => {
            match err {
                Error::Malformed(_) => println!("Malformed error"),
                Error::BadMagic(_) => println!("BadMagic error"),
                Error::Scroll(_) => println!("Scroll error"),
                Error::IO(_) => println!("IO error"),
            }
        }
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: goblin 
//  - Crate Link: None 
//  - Crate Version: 0.4.3 
//  - From Crate: crate_batch_2 
//  - From Crate Link: unknown_website 
//  - Module Path: goblin::archive::(Struct)Index 
//  - Use Statement: None 
//  - Function Name: parse_sysv_index 
//  - Function Usage: fn run_4() {
//     // ! goblin
//     let buffer: &[u8] = &([/* your data here */]);
//     if let Ok(index) = goblin::archive::Index::parse_sysv_index(buffer) {
//         println!("Parsed index: {:?}", index);
//     } else {
//         println!("Failed to parse index");
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a [u8]) -> Result<Index<'a>, Error>,
//   type_fields: [goblin::Index, goblin::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a [u8]) -> Result<Index<'a>, Error>,
//   type_fields: [goblin::Index, goblin::Error] 
// }
// Struct construction metadata: {
//   type_name: goblin::Index,
//   type_fields: [usize, alloc::Vec, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [flif::Metadata, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: flif::Metadata,
//   type_fields: [flif::ChunkType, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: flif::ChunkType,
//   type_fields: [Iccp, Exif, Exmp, Unknown([u8; {const}])] 
// }
// Struct construction metadata: {
//   type_name: goblin::Error,
//   type_fields: [Malformed(String), BadMagic(u64), Scroll(scroll::Error), IO(io::Error)] 
// }

