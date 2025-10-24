#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::io::{Cursor, Read, Seek};
use mp4ameta::Tag;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>
}

fuzz_target!(|input: FuzzInput| {
    let mut cursor = Cursor::new(input.data);
    let _ = Tag::read_from(&mut cursor);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: mp4ameta 
//  - Crate Link: None 
//  - Crate Version: 0.11.0 
//  - From Crate: crate_batch_3 
//  - From Crate Link: unknown_website 
//  - Module Path: mp4ameta::tag::(Struct)Tag 
//  - Use Statement: None 
//  - Function Name: read_from 
//  - Function Usage: fn run_9() {
//     //! this is the crashing output
//     // let data = [0, 0, 0, 1, 102, 116, 121, 112, 0, 132, 255, 255, 255, 255, 0, 132];
//     let data = [0, 1, 102, 116, 121, 112, 0, 132, 255, 255, 255, 255, 0, 132];
//     let mut data = std::io::Cursor::new(data);
//     let tag = mp4ameta::Tag::read_from(&mut data);
//     // suppose to be flif
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut impl Read + Seek) -> Result<Tag, Error>,
//   type_fields: [mp4ameta::Tag, mp4ameta::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut impl Read + Seek) -> Result<Tag, Error>,
//   type_fields: [mp4ameta::Tag, mp4ameta::Error] 
// }
// Struct construction metadata: {
//   type_name: mp4ameta::Tag,
//   type_fields: [alloc::String, mp4ameta::AudioInfo, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: alloc::String,
//   type_fields: [alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [alloc::RawVec, usize] 
// }
// Struct construction metadata: {
//   type_name: alloc::RawVec,
//   type_fields: [alloc::RawVecInner, core::PhantomData] 
// }
// Struct construction metadata: {
//   type_name: alloc::RawVecInner,
//   type_fields: [core::Unique, core::UsizeNoHighBit, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: core::Unique,
//   type_fields: [core::NonNull, core::PhantomData] 
// }
// Struct construction metadata: {
//   type_name: core::NonNull,
//   type_fields: [*const u8] 
// }
// Struct construction metadata: {
//   type_name: core::UsizeNoHighBit,
//   type_fields: [usize] 
// }
// Struct construction metadata: {
//   type_name: mp4ameta::AudioInfo,
//   type_fields: [core::Option, core::Option, core::Option, core::Option, core::Option] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: mp4ameta::Error,
//   type_fields: [mp4ameta::ErrorKind, alloc::String] 
// }
// Struct construction metadata: {
//   type_name: mp4ameta::ErrorKind,
//   type_fields: [AtomNotFound(Fourcc), DescriptorNotFound(u8), Io(io::Error), NoTag, Parsing, UnknownChannelConfig(u8), UnknownDataType(u32), UnknownMediaType(u8), UnknownVersion(u8), Utf8StringDecoding(string::FromUtf8Error), Utf16StringDecoding(string::FromUtf16Error), UnwritableData] 
// }

