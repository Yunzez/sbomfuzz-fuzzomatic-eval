#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use toml::Value; // Import using the public API of the crate

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _: Result<Value, _> = toml::from_str(s);
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: toml 
//  - Crate Link: https://github.com/alexcrichton/toml-rs 
//  - Crate Version: 0.4.1 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: toml::de 
//  - Use Statement: None 
//  - Function Name: from_str 
//  - Function Usage: fn run_4() {
//     println!("run 4");
// 
//     // ? line 197 - 200, and 202 logical bug, 203 infi recursion ,same funtion
//     let s = r#"
//          q = "\u000B"
//     "#;
//     let data: toml::Value = toml::from_str(s).unwrap();
//     println!("{:?}", data);
//     println!("{}", toml::to_string(&data).unwrap());
// 
//     // ? line 201
//     let s = r#"
//         "\n" = 5
//     "#;
//     let data: toml::Value = toml::from_str(s).unwrap();
//     println!("{:?}", data);
//     match toml::to_string(&data) {
//         Ok(serialized) => println!("{}", serialized),
//         Err(e) => eprintln!("Error serializing TOML: {}", e),
//     }
// 
//     // ? line 204, same bug as 203, but toml_edit, I reapeat the same function in toml
//     // ! crashing input
//     // let brackets = "[".repeat(20000);
//     let brackets = "[".repeat(2);
//     let input_string = format!("x={}", &brackets);
//     let _: Result<toml::Value, _> = toml::from_str(&input_string);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'de str) -> Result<T, Error>,
//   type_fields: [T, toml::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&'de str) -> Result<T, Error>,
//   type_fields: [T, toml::Error] 
// }
// Struct construction metadata: {
//   type_name: toml::Error,
//   type_fields: [alloc::Box] 
// }
// Struct construction metadata: {
//   type_name: alloc::Box,
//   type_fields: [toml::ErrorInner, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: toml::ErrorInner,
//   type_fields: [toml::ErrorKind, core::Option, usize, alloc::String, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: toml::ErrorKind,
//   type_fields: [UnexpectedEof, InvalidCharInString(char), InvalidEscape(char), InvalidHexEscape(char), InvalidEscapeValue(u32), NewlineInString, Unexpected(char), UnterminatedString, NewlineInTableKey, NumberInvalid, DateInvalid, Wanted, MixedArrayType, DuplicateTable(String), RedefineAsArray, EmptyTableKey, Custom, ExpectedString, __Nonexhaustive] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: alloc::String,
//   type_fields: [alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [tinytemplate::Instruction, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: tinytemplate::Instruction,
//   type_fields: [Literal(&'template str), Value(Path<'template>), FormattedValue(Path<'template>, &'template str), Branch(Path<'template>, bool, usize), PushNamedContext(Path<'template>, &'template str), PushIterationContext(Path<'template>, &'template str), PopContext, Iterate(usize), Goto(usize), Call(&'template str, Path<'template>)] 
// }

