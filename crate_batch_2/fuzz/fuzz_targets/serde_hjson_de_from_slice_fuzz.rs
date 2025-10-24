#![no_main]
use libfuzzer_sys::fuzz_target;
use serde_hjson::de::from_slice;
use arbitrary::Arbitrary;

fuzz_target!(|data: &[u8]| {
    let _result: Result<serde_hjson::Value, serde_hjson::Error> = from_slice(data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: serde_hjson 
//  - Crate Link: None 
//  - Crate Version: 0.10.0 
//  - From Crate: crate_batch_2 
//  - From Crate Link: unknown_website 
//  - Module Path: serde_hjson::de 
//  - Use Statement: None 
//  - Function Name: from_slice 
//  - Function Usage: fn run_6() {
//     // ! line 47 48 49 50 are the same function
//     let data: Vec<u8> = vec![155];
// 
//     let mut sample: Result<Map<String, Value>> = serde_hjson::from_slice(&data);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&[u8]) -> Result<T, Error>,
//   type_fields: [T, serde_hjson::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&[u8]) -> Result<T, Error>,
//   type_fields: [T, serde_hjson::Error] 
// }
// Struct construction metadata: {
//   type_name: serde_hjson::Error,
//   type_fields: [Syntax(ErrorCode, usize, usize), Io(io::Error), FromUtf8(FromUtf8Error)] 
// }

