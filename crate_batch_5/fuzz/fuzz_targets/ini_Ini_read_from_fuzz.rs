#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use ini::Ini;
use std::io::Cursor;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let mut cursor = Cursor::new(input.data);
    let _ = Ini::read_from(&mut cursor);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: ini 
//  - Crate Link: unknown_website 
//  - Crate Version: 0.16.0 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: ini::(Struct)Ini 
//  - Use Statement: None 
//  - Function Name: read_from 
//  - Function Usage: fn run_4() {
//     ini::Ini::read_from(&mut std::io::Cursor::new(String::from(""))).unwrap();
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut R) -> Result<Ini, Error>,
//   type_fields: [R, ini::Ini, ini::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut R) -> Result<Ini, Error>,
//   type_fields: [R, ini::Ini, ini::Error] 
// }
// Struct construction metadata: {
//   type_name: ini::Ini,
//   type_fields: [ordered_multimap::ListOrderedMultimap] 
// }
// Struct construction metadata: {
//   type_name: ordered_multimap::ListOrderedMultimap,
//   type_fields: [alloc::String, ini::Properties, std::RandomState] 
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
//   type_name: ini::Properties,
//   type_fields: [ordered_multimap::ListOrderedMultimap] 
// }
// Struct construction metadata: {
//   type_name: std::RandomState,
//   type_fields: [u64, u64] 
// }
// Struct construction metadata: {
//   type_name: ini::Error,
//   type_fields: [Io(io::Error), Parse(ParseError)] 
// }

