#![no_main]
extern crate libfuzzer_sys;
extern crate raven_uxn;

use libfuzzer_sys::fuzz_target;
use raven_uxn::Uxn;
use raven_uxn::UxnRam;
use raven_uxn::Backend;

fuzz_target!(|data: &[u8]| {
    let mut ram = UxnRam::new();
    let mut vm = Uxn::new(&mut ram, Backend::Interpreter);
    
    // Execute the reset function with the fuzzed input
    let _ = vm.reset(data);
});