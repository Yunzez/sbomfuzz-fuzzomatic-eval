#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use raven_uxn::Uxn;
use raven_uxn::UxnRam;
use raven_uxn::Backend;

fuzz_target!(|data: &[u8]| {
    let mut ram = UxnRam::new();
     let mut vm = Uxn::new(&mut ram, Backend::Interpreter);
    
    // Further operations on `ram` can be added here
     let _ = vm.reset(data);
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
//  - Module Path: raven_uxn::ram::(Struct)UxnRam 
//  - Use Statement: use raven_uxn::UxnRam 
//  - Function Name: new 
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
//   type_name: fn() -> UxnRam,
//   type_fields: [raven_uxn::UxnRam] 
// }
// Struct construction metadata: {
//   type_name: fn() -> UxnRam,
//   type_fields: [raven_uxn::UxnRam] 
// }
// Struct construction metadata: {
//   type_name: raven_uxn::UxnRam,
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

