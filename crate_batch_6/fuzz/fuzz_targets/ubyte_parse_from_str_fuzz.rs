#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use ubyte::ByteUnit;
use std::str::FromStr;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: String,
}

fuzz_target!(|input: FuzzInput| {
    // Attempt to parse the input as a ByteUnit
    let _ = ByteUnit::from_str(&input.data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: ubyte 
//  - Crate Link: None 
//  - Crate Version: 0.10.1 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: ubyte::parse::(Struct)ByteUnit 
//  - Use Statement: None 
//  - Function Name: from_str 
//  - Function Usage: fn run_6() {
//     println!("run 6");
//     let data = "1 KB";
//     match data.parse::<ubyte::ByteUnit>() {
//         Ok(byte_unit) => println!("Parsed byte unit: {:?}", byte_unit),
//         Err(e) => eprintln!("Error parsing byte unit: {:?}", e),
//     }
// } 
//  - Parameters: initial function signature:No type_fields: fn(&str) -> Result<ByteUnit, <ByteUnit as FromStr>::Err>

