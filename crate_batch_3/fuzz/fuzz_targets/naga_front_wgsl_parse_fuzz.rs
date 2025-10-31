#![no_main]
use libfuzzer_sys::fuzz_target;
use naga::front::wgsl::Parser;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a [u8],
}

fuzz_target!(|input: FuzzInput| {
    if let Ok(data_str) = std::str::from_utf8(input.data) {
        let mut parser = Parser::new();
        let _ = parser.parse(data_str);
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: naga 
//  - Crate Link: https://github.com/gfx-rs/naga 
//  - Crate Version: 0.1.0 
//  - From Crate: crate_batch_3 
//  - From Crate Link: unknown_website 
//  - Module Path: naga::front::wgsl::(Struct)Parser 
//  - Use Statement: None 
//  - Function Name: parse 
//  - Function Usage: fn run_10() {
//     // suppose to be flif
//     let data = [0, 0, 0, 1, 102, 116, 121, 112, 0, 132, 255, 255, 255, 255, 0, 132];
//     let data_str = std::str::from_utf8(&data).unwrap_or("");
//     let _result = Parser::new().parse(data_str);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut Parser, &'a str) -> Result<Module, ParseError<'a>>,
//   type_fields: [naga::Parser, naga::Module, naga::ParseError] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut Parser, &'a str) -> Result<Module, ParseError<'a>>,
//   type_fields: [naga::Parser, naga::Module, naga::ParseError] 
// }
// Struct construction metadata: {
//   type_name: naga::Parser,
//   type_fields: [alloc::Vec, std::HashMap, core::Option] 
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
//   type_name: std::HashMap,
//   type_fields: [lopdf::Bookmark, std::RandomState] 
// }
// Struct construction metadata: {
//   type_name: lopdf::Bookmark,
//   type_fields: [alloc::Vec, alloc::String, u32, [f32; 3], (u32, u16), u32] 
// }
// Struct construction metadata: {
//   type_name: alloc::String,
//   type_fields: [alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: std::RandomState,
//   type_fields: [u64, u64] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: naga::Module,
//   type_fields: [naga::Header, naga::Arena, naga::Arena, naga::Arena, naga::Arena, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: naga::Header,
//   type_fields: [(u8, u8, u8), u32] 
// }
// Struct construction metadata: {
//   type_name: naga::Arena,
//   type_fields: [alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: naga::ParseError,
//   type_fields: [naga::Error, alloc::Vec, (usize, usize)] 
// }
// Struct construction metadata: {
//   type_name: naga::Error,
//   type_fields: [Unexpected(Token<'a>), UnexpectedConstantType(crate::proc::UnexpectedConstantTypeError), BadInteger(&'a str, std::num::ParseIntError), BadFloat(&'a str, std::num::ParseFloatError), BadAccessor(&'a str), InvalidResolve(ResolveError), UnknownImport(&'a str), UnknownStorageClass(&'a str), UnknownDecoration(&'a str), UnknownBuiltin(&'a str), UnknownShaderStage(&'a str), UnknownIdent(&'a str), UnknownType(&'a str), UnknownFunction(&'a str), MissingMemberOffset(&'a str), MutabilityViolation(&'a str), Other] 
// }

