#![no_main]
use libfuzzer_sys::fuzz_target;
use lopdf::Document;

fuzz_target!(|data: &[u8]| {
    let _ = Document::load_mem(data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: lopdf 
//  - Crate Link: https://github.com/J-F-Liu/lopdf 
//  - Crate Version: 0.26.0 
//  - From Crate: crate_batch_3 
//  - From Crate Link: unknown_website 
//  - Module Path: lopdf::reader::(Struct)Document 
//  - Use Statement: None 
//  - Function Name: load_mem 
//  - Function Usage: fn run_2() {
//     let d = b"%PDF-1.5\n\
//     000000028100 000 n \n\
//     0000000338 00000 n \n\
//     %%EOF";
//     let _ = lopdf::Document::load_mem(d);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&[u8]) -> Result<Document, Error>,
//   type_fields: [lopdf::Document, lopdf::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&[u8]) -> Result<Document, Error>,
//   type_fields: [lopdf::Document, lopdf::Error] 
// }
// Struct construction metadata: {
//   type_name: lopdf::Document,
//   type_fields: [alloc::String, lopdf::Dictionary, lopdf::Xref, alloc::BTreeMap, u32, u32, alloc::Vec, std::HashMap] 
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
//   type_name: lopdf::Dictionary,
//   type_fields: [linked_hash_map::LinkedHashMap] 
// }
// Struct construction metadata: {
//   type_name: linked_hash_map::LinkedHashMap,
//   type_fields: [alloc::Global, lopdf::Object, std::RandomState] 
// }
// Struct construction metadata: {
//   type_name: lopdf::Object,
//   type_fields: [Null, Boolean(bool), Integer(i64), Real(f64), Name(Vec<u8>), String(Vec<u8>, StringFormat), Array(Vec<Object>), Dictionary(Dictionary), Stream(Stream), Reference(ObjectId)] 
// }
// Struct construction metadata: {
//   type_name: std::RandomState,
//   type_fields: [u64, u64] 
// }
// Struct construction metadata: {
//   type_name: lopdf::Xref,
//   type_fields: [alloc::BTreeMap, u32] 
// }
// Struct construction metadata: {
//   type_name: alloc::BTreeMap,
//   type_fields: [lopdf::Object, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: std::HashMap,
//   type_fields: [lopdf::Bookmark, std::RandomState] 
// }
// Struct construction metadata: {
//   type_name: lopdf::Bookmark,
//   type_fields: [alloc::Vec, alloc::String, u32, [f32; 3], (u32, u16), u32] 
// }
// Struct construction metadata: {
//   type_name: lopdf::Error,
//   type_fields: [ContentDecode, DictKey, Header, IO(std::io::Error), ObjectIdMismatch, ObjectNotFound, Offset(usize), PageNumberNotFound(u32), Parse, ReferenceLimit, BracketLimit, Trailer, Type, UTF8, Syntax(String), Xref(XrefError)] 
// }

