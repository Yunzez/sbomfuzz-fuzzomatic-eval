#![no_main]
use libfuzzer_sys::fuzz_target;
use rustc_demangle::demangle;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    mangled: String,
}

fuzz_target!(|data: FuzzInput| {
    let mangled_str = data.mangled;
    let _ = std::panic::catch_unwind(|| {
        demangle(&mangled_str);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: rustc_demangle 
//  - Crate Link: https://github.com/alexcrichton/rustc-demangle 
//  - Crate Version: 0.1.5 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: rustc_demangle 
//  - Use Statement: None 
//  - Function Name: demangle 
//  - Function Usage: fn run_5() {
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             rustc_demangle::demangle("_ZN2222222222222222222222EE"); // should panic
//         })
//     {
//         eprintln!("Caught panic in run 5: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Demangle<'_>,
//   type_fields: [rustc_demangle::Demangle] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Demangle<'_>,
//   type_fields: [rustc_demangle::Demangle] 
// }
// Struct construction metadata: {
//   type_name: rustc_demangle::Demangle,
//   type_fields: [&str, &str, bool, usize] 
// }

