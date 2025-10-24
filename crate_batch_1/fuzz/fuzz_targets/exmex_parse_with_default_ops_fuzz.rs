#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use exmex::{self, ExParseError};

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    expression: &'a str,
}

fuzz_target!(|data: FuzzInput| {
    let _ = exmex::parse_with_default_ops::<f64>(data.expression);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: exmex 
//  - Crate Link: https://github.com/bertiqwerty/exmex/ 
//  - Crate Version: 0.8.4 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: exmex 
//  - Use Statement: None 
//  - Function Name: parse_with_default_ops 
//  - Function Usage: fn run_14() {
//     let s = "3 + 4 * 2";
//     let _ = exmex::parse_with_default_ops::<f64>(s).unwrap();
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a str) -> Result<FlatEx<'a, T>, ExParseError>,
//   type_fields: [T, exmex::ExParseError] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a str) -> Result<FlatEx<'a, T>, ExParseError>,
//   type_fields: [T, exmex::ExParseError] 
// }
// Struct construction metadata: {
//   type_name: exmex::ExParseError,
//   type_fields: [alloc::String] 
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

