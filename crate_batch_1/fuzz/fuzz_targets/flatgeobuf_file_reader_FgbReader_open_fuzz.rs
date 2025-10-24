#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::Cursor;
use arbitrary::Arbitrary;
use flatgeobuf::FgbReader;
use std::io::BufReader;
use std::io::Read;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let cursor = Cursor::new(input.data);
    let mut reader = BufReader::new(cursor);
    
    let _ = FgbReader::open(&mut reader);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: flatgeobuf 
//  - Crate Link: https://bjornharrtell.github.io/flatgeobuf/ 
//  - Crate Version: 0.3.4 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: flatgeobuf::file_reader::(Struct)FgbReader 
//  - Use Statement: None 
//  - Function Name: header 
//  - Function Usage: fn run_17() {
//     let mut input: &[u8] = &[102, 103, 98, 3, 102, 103, 98, 0, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 219, 216, 216, 216, 216, 216, 216, 216, 39, 39, 39, 39, 39, 32, 39, 39, 39, 39, 39, 39, 39, 39, 39, 10, 169, 247, 247, 247, 247];
//     let mut buf_reader = BufReader::new(Cursor::new(input));
// 
//     // ! spreedsheet line 36
//     let mut fgb = match FgbReader::open(&mut buf_reader) {
//         Ok(n) => n,
//         Err(_) => return,
//     };
//     // ! spreedsheet line 37
//     let _ = fgb.header();
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&FgbReader<'a>) -> Header<'_>,
//   type_fields: [flatgeobuf::FgbReader, flatgeobuf::Header] 
// }
// Struct construction metadata: {
//   type_name: fn(&FgbReader<'a>) -> Header<'_>,
//   type_fields: [flatgeobuf::FgbReader, flatgeobuf::Header] 
// }
// Struct construction metadata: {
//   type_name: flatgeobuf::FgbReader,
//   type_fields: [&'a mut (dyn ReadSeek + 'static), flatgeobuf::FgbFeature, u64, core::Option, usize, usize] 
// }
// Struct construction metadata: {
//   type_name: flatgeobuf::FgbFeature,
//   type_fields: [alloc::Vec, alloc::Vec] 
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
//   type_fields: [*const str] 
// }
// Struct construction metadata: {
//   type_name: core::UsizeNoHighBit,
//   type_fields: [usize] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: flatgeobuf::Header,
//   type_fields: [flatbuffers::Table] 
// }
// Struct construction metadata: {
//   type_name: flatbuffers::Table,
//   type_fields: [&[u8], usize] 
// }

