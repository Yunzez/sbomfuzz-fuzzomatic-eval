
#![no_main]
use libfuzzer_sys::fuzz_target;
use rust_minidump_86::Minidump;
use arbitrary::Arbitrary;
use std::ops::Deref;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

impl Deref for FuzzInput {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fuzz_target!(|input: FuzzInput| {
    // Execute the fuzz target with the provided inputs.
    match Minidump::read(input) {
        Ok(f) => {
            match f.get_stream::<rust_minidump_86::MinidumpLinuxMaps>() {
                Ok(maps) => { let _ = maps; },
                Err(e) => { assert_eq!(e.to_string(), "Data error"); }
            };
        }
        Err(_) => {}
    }
});
