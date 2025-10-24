#![no_main]
extern crate libfuzzer_sys;
extern crate pdf_115;
use libfuzzer_sys::fuzz_target;
use pdf_115::file::File;
use libfuzzer_sys::arbitrary::{self, Arbitrary};

fuzz_target!(|data: Vec<u8>| {
    if let Ok(path) = std::str::from_utf8(&data) {
        if let Ok(file) = File::<Vec<u8>>::open(path) {
            for i in 0..file.num_pages() {
                let _ = file.get_page(i);
            }
        }
    }
});