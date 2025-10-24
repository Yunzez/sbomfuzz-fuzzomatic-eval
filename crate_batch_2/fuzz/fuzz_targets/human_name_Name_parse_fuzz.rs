#![no_main]
use libfuzzer_sys::fuzz_target;
use human_name::Name;
use arbitrary::Arbitrary;

fuzz_target!(|input: &str| {
    // Fuzz the parse function of human_name::Name
    let _ = Name::parse(input);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: human_name 
//  - Crate Link: unknown_website 
//  - Crate Version: 1.3.1 
//  - From Crate: crate_batch_2 
//  - From Crate Link: unknown_website 
//  - Module Path: human_name::(Struct)Name 
//  - Use Statement: None 
//  - Function Name: parse 
//  - Function Usage: fn run_7() {
//     // human-name
//     let name = human_name::Name::parse(".ΰ\u{330}\u{610}`");
// } 
//  - Parameters: initial function signature:No type_fields: fn(&str) -> Option<Name>

