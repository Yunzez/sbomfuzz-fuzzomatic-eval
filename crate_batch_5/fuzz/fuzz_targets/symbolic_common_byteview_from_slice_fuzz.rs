#![no_main]
use libfuzzer_sys::fuzz_target;
use symbolic::common::ByteView;
use arbitrary::Arbitrary;

#[derive(Debug, Arbitrary)]
struct FuzzInput<'a> {
    data: &'a [u8],
}

fuzz_target!(|input: FuzzInput| {
    let _bv = ByteView::from_slice(input.data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: symbolic_common 
//  - Crate Link: https://github.com/getsentry/symbolic 
//  - Crate Version: 12.1.2 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: symbolic_common::byteview::(Struct)ByteView 
//  - Use Statement: use symbolic::common::ByteView; 
//  - Function Name: from_slice 
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
//   type_name: fn(&'a [u8]) -> ByteView<'a>,
//   type_fields: [symbolic_common::ByteView] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a [u8]) -> ByteView<'a>,
//   type_fields: [symbolic_common::ByteView] 
// }
// Struct construction metadata: {
//   type_name: symbolic_common::ByteView,
//   type_fields: [alloc::Arc] 
// }
// Struct construction metadata: {
//   type_name: alloc::Arc,
//   type_fields: [symbolic_common::ByteViewBacking, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: symbolic_common::ByteViewBacking,
//   type_fields: [Buf(Cow<'a, [u8]>), Mmap(Mmap)] 
// }

