#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};
use raven_uxn::{Uxn, UxnRam, Backend};

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a [u8],
}

fuzz_target!(|fuzz_input: FuzzInput| {
    let mut ram = UxnRam::new();
    let mut vm = Uxn::new(&mut ram, Backend::Interpreter);

    let result = vm.reset(fuzz_input.data);
    // Further assertions or operations can be done on `result` if required
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: raven_uxn 
//  - Crate Link: None 
//  - Crate Version: 0.1.0 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: raven_uxn::(Struct)Uxn 
//  - Use Statement: None 
//  - Function Name: reset 
//  - Function Usage: fn run_update() {
//     let mut ram_v = UxnRam::new();
//     let mut vm_v = Uxn::new(&mut ram_v, Backend::Interpreter);
// 
// 
//     // Don't load any programs that require auxiliary memory
//     let data: &[u8] = &[0x00, 0x01, 0x02, 0x03]; // Random data
//     if !vm_v.reset(data).is_empty() {
//         return;
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut Uxn<'a>, &'b [u8]) -> &'b [u8],
//   type_fields: [raven_uxn::Uxn] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut Uxn<'a>, &'b [u8]) -> &'b [u8],
//   type_fields: [raven_uxn::Uxn] 
// }
// Struct construction metadata: {
//   type_name: raven_uxn::Uxn,
//   type_fields: [[u8; 256], &'a mut [u8; 65536], raven_uxn::Stack, raven_uxn::Stack, raven_uxn::Backend] 
// }
// Struct construction metadata: {
//   type_name: raven_uxn::Stack,
//   type_fields: [[u8; 256], u8] 
// }
// Struct construction metadata: {
//   type_name: raven_uxn::Backend,
//   type_fields: [Interpreter] 
// }

