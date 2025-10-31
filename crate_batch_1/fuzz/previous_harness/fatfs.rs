#![no_main]
extern crate libfuzzer_sys;
extern crate fatfs;

use libfuzzer_sys::fuzz_target;
use fatfs::{FileSystem, FsOptions};
use std::io::Cursor;
use arbitrary::Arbitrary;

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input:  &[u8]| {
    let mut storage = Cursor::new(input.to_vec());
    let _ = FileSystem::new(&mut storage, FsOptions::new()).ok();
});

