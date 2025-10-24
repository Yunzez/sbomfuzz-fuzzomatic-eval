#![no_main]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use rust_minidump_86::Minidump;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let _ = Minidump::read(input.data);
});