#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use tera_190::{Tera, Context};

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    template: String,
    flag: bool,
}

fuzz_target!(|data: FuzzInput| {
    let context = Context::new();
    
    let _ = std::panic::catch_unwind(|| {
        let _ = Tera::one_off(&data.template, &context, data.flag);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: tera 
//  - Crate Link: https://github.com/Keats/tera 
//  - Crate Version: 0.11.20 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: tera::tera::(Struct)Tera 
//  - Use Statement: None 
//  - Function Name: one_off 
//  - Function Usage: fn run_19() {
//     // ? line 190 191, same bug function
//     let context = tera_190::Context::new();
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             let _ = tera_190::Tera::one_off("{{1/0}}", &context, true);
//         })
//     {
//         eprintln!("Caught panic in run 19: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str, &T, bool) -> Result<String, Error>,
//   type_fields: [T, alloc::String, tera::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&str, &T, bool) -> Result<String, Error>,
//   type_fields: [T, alloc::String, tera::Error] 
// }
// Struct construction metadata: {
//   type_name: alloc::String,
//   type_fields: [alloc::Vec] 
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
//   type_name: tera::Error,
//   type_fields: [tera::ErrorKind, error_chain::State] 
// }
// Struct construction metadata: {
//   type_name: tera::ErrorKind,
//   type_fields: [Json(serde_json::Error), Msg(String), __Nonexhaustive] 
// }
// Struct construction metadata: {
//   type_name: error_chain::State,
//   type_fields: [core::Option, error_chain::InternalBacktrace] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: error_chain::InternalBacktrace,
//   type_fields: [core::Option] 
// }

