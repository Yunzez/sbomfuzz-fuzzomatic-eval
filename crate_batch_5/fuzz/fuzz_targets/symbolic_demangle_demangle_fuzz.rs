#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::borrow::Cow;
use symbolic::demangle::demangle;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: String,
}

fuzz_target!(|input: FuzzInput| {
    let _ = demangle(&input.data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: symbolic_demangle 
//  - Crate Link: https://github.com/getsentry/symbolic 
//  - Crate Version: 12.1.2 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: symbolic_demangle 
//  - Use Statement: None 
//  - Function Name: demangle 
//  - Function Usage: // use symbolic::minidump::processor::ProcessState;
// fn run_16() {
//     println!("run 16");
//     // ? line 184
//     symbolic::demangle::demangle(
//         "_ZUlzjjlZZL1zStUlSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjtUlSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjzL4t7IjIjjzjjzSt7j_Z3kjIIjfjzStfjzSt7j_ZA3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjzL4t7IjIjjzjjzSt7j_Z3kjIIjfjzStfjzSt7j_ZA3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjzL4t7IjIjL1vfIIEEEjzjjSI"
//     );
// 
//     let data =
//         b"MDMP\x93\xa7\x00\x00\r\x00\x00\x00 \xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
//     let bv = ByteView::from_slice(data);
//     // ProcessState::from_minidump(&bv, None);
// 
//     // ? line 186
// 
//     let data =
//         b"MDMP\x93\xa7\x00\x00\r\x00\x00\x00 \xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
//     let _ = symbolic::unreal::Unreal4Crash::parse_with_limit(data, 1024 * 1024);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Cow<'_, str>,
//   type_fields: [alloc::Cow] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Cow<'_, str>,
//   type_fields: [alloc::Cow] 
// }
// Struct construction metadata: {
//   type_name: alloc::Cow,
//   type_fields: [Borrowed(&'a B), Owned(<B as ToOwned>::Owned)] 
// }

