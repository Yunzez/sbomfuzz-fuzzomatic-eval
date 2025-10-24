#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(line) = str::from_utf8(data) {
        let _: Result<todotxt::Task, _> = line.parse();
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: todotxt 
//  - Crate Link: None 
//  - Crate Version: 0.3.0 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: todotxt::(Struct)Task 
//  - Use Statement: None 
//  - Function Name: from_str 
//  - Function Usage: fn run_2() {
//     println!("run 2");
//     let data = b"2021-01-01 This is a task +project @context";
//     if let Ok(line) = std::str::from_utf8(data) {
//         let _: Result<Task, _> = line.parse();
//     }
// } 
//  - Parameters: initial function signature:No type_fields: fn(&str) -> Result<Task, ()>

