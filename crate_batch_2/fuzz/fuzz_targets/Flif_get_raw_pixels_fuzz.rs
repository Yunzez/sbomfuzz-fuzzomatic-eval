#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use flif::Flif;
use std::io::Cursor;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    if let Ok(img) = Flif::decode(Cursor::new(&input.data)) {
        let _ = img.get_raw_pixels();
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: flif 
//  - Crate Link: None 
//  - Crate Version: 0.2.0 
//  - From Crate: crate_batch_2 
//  - From Crate Link: unknown_website 
//  - Module Path: flif::(Struct)Flif 
//  - Use Statement: None 
//  - Function Name: get_raw_pixels 
//  - Function Usage: fn run_1() {
//     let paths = std::fs::read_dir(".").unwrap();
//     for path in paths {
//         let path = path.unwrap().path();
//         println!("Artifact: {}", path.display());
//         let mut data = Vec::new();
//         let mut file = File::open(path).unwrap();
//         file.read_to_end(&mut data).unwrap();
//         // temporarily disabled
//         let _ = Flif::decode(Cursor::new(&data)).map(|img| img.get_raw_pixels());
//     }
// 
// 
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&Flif) -> Vec<u8, Global>,
//   type_fields: [flif::Flif, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: fn(&Flif) -> Vec<u8, Global>,
//   type_fields: [flif::Flif, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: flif::Flif,
//   type_fields: [flif::FlifInfo, flif::DecodingImage] 
// }
// Struct construction metadata: {
//   type_name: flif::FlifInfo,
//   type_fields: [flif::Header, alloc::Vec, flif::SecondHeader, alloc::Box] 
// }
// Struct construction metadata: {
//   type_name: flif::Header,
//   type_fields: [bool, flif::ColorSpace, flif::BytesPerChannel, usize, usize, u32] 
// }
// Struct construction metadata: {
//   type_name: flif::ColorSpace,
//   type_fields: [Monochrome, RGB, RGBA] 
// }
// Struct construction metadata: {
//   type_name: flif::BytesPerChannel,
//   type_fields: [Custom, One, Two] 
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
//   type_name: flif::SecondHeader,
//   type_fields: [alloc::Vec, bool, core::Option, core::Option, bool, u8, u8, bool, alloc::Vec, core::Option] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: alloc::Box,
//   type_fields: [dyn Transform + 'static, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: flif::DecodingImage,
//   type_fields: [usize, usize, flif::ColorSpace, alloc::Vec] 
// }

