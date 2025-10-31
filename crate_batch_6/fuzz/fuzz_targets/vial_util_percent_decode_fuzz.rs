#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use vial::util::percent_decode;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: String,
}

fuzz_target!(|fuzz_input: FuzzInput| {
    let _ = percent_decode(&fuzz_input.data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: vial 
//  - Crate Link: https://vial.rs 
//  - Crate Version: 0.1.9 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: vial::util 
//  - Use Statement: None 
//  - Function Name: percent_decode 
//  - Function Usage: fn run_11() {
//     println!("run 11");
//     let x = vial::Request::from_reader(std::io::empty());
//     match x {
//         Ok(request) => {
//             vial::util::percent_decode(request.path());
//         }
//         Err(e) => eprintln!("Error creating request: {:?}", e),
//     }
// } 
//  - Parameters: initial function signature:No type_fields: fn(&str) -> Option<String>

