#![no_main]
use libfuzzer_sys::fuzz_target;
use npy::from_bytes;
use arbitrary::{Arbitrary, Unstructured};
use std::vec::Vec;
extern crate npy;

#[macro_use]
extern crate npy_derive;
#[derive(Debug, NpyData)]
struct Array {
    a: i32,
}

fuzz_target!(|data: &[u8]| {
    let _ = from_bytes::<Array>(&data);
});
