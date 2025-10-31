#![no_main]
use libfuzzer_sys::fuzz_target;
use syn_188::parse_str;
use syn_188::Expr;
use std::panic;

fuzz_target!(|data: &str| {
    let _ = panic::catch_unwind(|| {
        let _result = parse_str::<Expr>(data);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: syn 
//  - Crate Link: None 
//  - Crate Version: 1.0.41 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: syn 
//  - Use Statement: None 
//  - Function Name: parse_str 
//  - Function Usage: fn run_18() {
//     // ? same bug function for 188 and 189
//     println!("run 18");
//     let s = "6E--5458";
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             let _ = syn_188::parse_str::<Expr>(s);
//         })
//     {
//         eprintln!("Caught panic in run 18: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Result<T, Error>,
//   type_fields: [T, serde_yaml::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Result<T, Error>,
//   type_fields: [T, serde_yaml::Error] 
// }
// Struct construction metadata: {
//   type_name: serde_yaml::Error,
//   type_fields: [alloc::Box] 
// }
// Struct construction metadata: {
//   type_name: alloc::Box,
//   type_fields: [serde_yaml::ErrorImpl, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: serde_yaml::ErrorImpl,
//   type_fields: [Message(String, Option<Pos>), Emit(emitter::EmitError), Scan(scanner::ScanError), Io(io::Error), Utf8(str::Utf8Error), FromUtf8(string::FromUtf8Error), EndOfStream, MoreThanOneDocument, RecursionLimitExceeded, Shared(Arc<ErrorImpl>)] 
// }

