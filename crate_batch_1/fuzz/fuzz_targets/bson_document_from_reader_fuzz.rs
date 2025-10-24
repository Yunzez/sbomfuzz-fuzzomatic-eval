#![no_main]
use libfuzzer_sys::fuzz_target;
use bson_10::Document;
use std::io::Cursor;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let mut reader = Cursor::new(&input.data);
    let _ = bson_10::decode_document(&mut reader);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: bson 
//  - Crate Link: https://github.com/mongodb/bson-rust 
//  - Crate Version: 1.2.0 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: bson::document::(Struct)Document 
//  - Use Statement: None 
//  - Function Name: from_reader 
//  - Function Usage: fn run_5() {
//     let buf = vec![0u8; 128]; // Example buffer, replace with actual data
//     bson_10::decode_document(&mut Cursor::new(&buf[..]));
//     bson_11::decode_document(&mut Cursor::new(&buf[..]));
// 
//     let crash = b"\n\x00\x00\x00\x04\x00\x00\x00\x00\x00";
//     let mut reader = std::io::Cursor::new(crash);
//     let _ = bson_12::Document::from_reader(&mut reader);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut R) -> Result<Document, Error>,
//   type_fields: [R, bson::Document, bson::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut R) -> Result<Document, Error>,
//   type_fields: [R, bson::Document, bson::Error] 
// }
// Struct construction metadata: {
//   type_name: bson::Document,
//   type_fields: [linked_hash_map::LinkedHashMap] 
// }
// Struct construction metadata: {
//   type_name: linked_hash_map::LinkedHashMap,
//   type_fields: [alloc::String, bson::Bson, std::RandomState] 
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
//   type_fields: [*const str] 
// }
// Struct construction metadata: {
//   type_name: core::UsizeNoHighBit,
//   type_fields: [usize] 
// }
// Struct construction metadata: {
//   type_name: bson::Bson,
//   type_fields: [FloatingPoint(f64), String(String), Array(Array), Document(Document), Boolean(bool), Null, RegExp(String, String), JavaScriptCode(String), JavaScriptCodeWithScope(String, Document), I32(i32), I64(i64), TimeStamp(i64), Binary(BinarySubtype, Vec<u8>), ObjectId(oid::ObjectId), UtcDatetime(DateTime<Utc>), Symbol(String)] 
// }
// Struct construction metadata: {
//   type_name: std::RandomState,
//   type_fields: [u64, u64] 
// }
// Struct construction metadata: {
//   type_name: bson::Error,
//   type_fields: [IoError(io::Error), FromUtf8Error(string::FromUtf8Error), UnrecognizedDocumentElementType, SyntaxError, EndOfStream, InvalidTimestamp(i64), AmbiguousTimestamp(i64), DeserializationError] 
// }

