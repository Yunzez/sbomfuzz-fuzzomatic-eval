#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::io::Cursor;
use plist::Value;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let cursor = Cursor::new(input.data);
    let _ = Value::from_reader(cursor);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: plist 
//  - Crate Link: None 
//  - Crate Version: 1.2.2 
//  - From Crate: crate_batch_4 
//  - From Crate Link: unknown_website 
//  - Module Path: plist::value::(Enum)Value 
//  - Use Statement: None 
//  - Function Name: from_reader 
//  - Function Usage: fn run_7() {
//     // info: plist
//     println!("running plist");
// 
//     // ! crashing data
//     // let data = b"bplist00\x10\x01\x00\x00\x00\x00\x00\x003~\x00\x00\x00\x00\x00\x00\x00\x01\x02\x00\x00\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00";
// 
//     let data = b"bplist00\x10\x01";
// 
//     let cursor = Cursor::new(data);
//     plist::Value::from_reader(cursor);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(R) -> Result<Value, Error>,
//   type_fields: [R, plist::Value, plist::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(R) -> Result<Value, Error>,
//   type_fields: [R, plist::Value, plist::Error] 
// }
// Struct construction metadata: {
//   type_name: plist::Value,
//   type_fields: [Array(Vec<Value>), Dictionary(Dictionary), Boolean(bool), Data(Vec<u8>), Date(Date), Real(f64), Integer(Integer), String(String), Uid(Uid), __Nonexhaustive] 
// }
// Struct construction metadata: {
//   type_name: plist::Error,
//   type_fields: [alloc::Box] 
// }
// Struct construction metadata: {
//   type_name: alloc::Box,
//   type_fields: [plist::ErrorImpl, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: plist::ErrorImpl,
//   type_fields: [plist::ErrorKind, core::Option] 
// }
// Struct construction metadata: {
//   type_name: plist::ErrorKind,
//   type_fields: [UnexpectedEof, UnexpectedEndOfEventStream, UnexpectedEventType, UnclosedXmlElement, UnpairedXmlClosingTag, UnexpectedXmlCharactersExpectedElement, UnexpectedXmlOpeningTag, UnknownXmlElement, InvalidXmlSyntax, InvalidXmlUtf8, InvalidDataString, InvalidDateString, InvalidIntegerString, InvalidRealString, UidNotSupportedInXmlPlist, ObjectTooLarge, InvalidMagic, InvalidTrailerObjectOffsetSize, InvalidTrailerObjectReferenceSize, InvalidObjectLength, ObjectReferenceTooLarge, ObjectOffsetTooLarge, RecursiveObject, NullObjectUnimplemented, FillObjectUnimplemented, IntegerOutOfRange, InfiniteOrNanDate, InvalidUtf8String, InvalidUtf16String, UnknownObjectType(u8), Io(io::Error), Serde(String)] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }

