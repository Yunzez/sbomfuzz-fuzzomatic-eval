#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use swf_parser::streaming::movie;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let _ = std::panic::catch_unwind(|| {
        let _ = movie::parse_movie(&input.data[..]);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: swf_parser 
//  - Crate Link: https://github.com/open-flash/swf-parser 
//  - Crate Version: 0.9.0 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: swf_parser::streaming::movie 
//  - Use Statement: use swf_parser::streaming::movie; 
//  - Function Name: parse_movie 
//  - Function Usage: fn run_15() {
//     let bytes = b"CWSCCCACCGCCC";
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             let _ = movie::parse_movie(&bytes[..]);
//         })
//     {
//         eprintln!("Caught panic in run 15: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&[u8]) -> Result<(&[u8], Movie), Err<(&[u8], ErrorKind)>>,
//   type_fields: [swf_tree::Movie, nom::ErrorKind] 
// }
// Struct construction metadata: {
//   type_name: fn(&[u8]) -> Result<(&[u8], Movie), Err<(&[u8], ErrorKind)>>,
//   type_fields: [swf_tree::Movie, nom::ErrorKind] 
// }
// Struct construction metadata: {
//   type_name: swf_tree::Movie,
//   type_fields: [swf_tree::Header, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: swf_tree::Header,
//   type_fields: [u8, swf_tree::Rect, swf_fixed::Ufixed8P8, u16] 
// }
// Struct construction metadata: {
//   type_name: swf_tree::Rect,
//   type_fields: [i32, i32, i32, i32] 
// }
// Struct construction metadata: {
//   type_name: swf_fixed::Ufixed8P8,
//   type_fields: [u16] 
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
//   type_name: nom::ErrorKind,
//   type_fields: [Tag, MapRes, MapOpt, Alt, IsNot, IsA, SeparatedList, SeparatedNonEmptyList, Many0, Many1, ManyTill, Count, TakeUntil, LengthValue, TagClosure, Alpha, Digit, HexDigit, OctDigit, AlphaNumeric, Space, MultiSpace, LengthValueFn, Eof, Switch, TagBits, OneOf, NoneOf, Char, CrLf, RegexpMatch, RegexpMatches, RegexpFind, RegexpCapture, RegexpCaptures, TakeWhile1, Complete, Fix, Escaped, EscapedTransform, NonEmpty, ManyMN, Not, Permutation, Verify, TakeTill1, TakeWhileMN, ParseTo, TooLarge, Many0Count, Many1Count, Float] 
// }

