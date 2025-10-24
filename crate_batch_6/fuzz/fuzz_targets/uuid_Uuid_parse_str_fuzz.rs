#![no_main]
use libfuzzer_sys::fuzz_target;
use uuid::Uuid;
use std::panic;

fuzz_target!(|data: &str| {
    let _ = panic::catch_unwind(|| {
        let _ = Uuid::parse_str(data);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: uuid 
//  - Crate Link: https://github.com/rust-lang/uuid 
//  - Crate Version: 0.4.0 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: uuid::(Struct)Uuid 
//  - Use Statement: None 
//  - Function Name: parse_str 
//  - Function Usage: fn run_10() {
//     println!("run 10");
//     let result = panic::catch_unwind(||uuid::Uuid::parse_str("F9168C5E-CEB2F4faaFB6BFF329BF39FA1E4").unwrap());
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Result<Uuid, ParseError>,
//   type_fields: [uuid::Uuid, uuid::ParseError] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Result<Uuid, ParseError>,
//   type_fields: [uuid::Uuid, uuid::ParseError] 
// }
// Struct construction metadata: {
//   type_name: uuid::Uuid,
//   type_fields: [[u8; 16]] 
// }
// Struct construction metadata: {
//   type_name: uuid::ParseError,
//   type_fields: [InvalidLength(usize), InvalidCharacter(char, usize), InvalidGroups(usize), InvalidGroupLength(usize, usize, u8)] 
// }

