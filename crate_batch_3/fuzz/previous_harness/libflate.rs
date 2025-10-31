#![no_main]
extern crate libfuzzer_sys;
use libflate::deflate::Decoder;
use libfuzzer_sys::fuzz_target;
use std::io::{self, Read};

fuzz_target!(|data: &[u8]| {
    let mut decoder = Decoder::new(data);
    let _ = io::copy(&mut decoder, &mut io::sink());
});