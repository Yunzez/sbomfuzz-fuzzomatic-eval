
#![no_main]
extern crate libfuzzer_sys;
extern crate pdf_115;
use libfuzzer_sys::fuzz_target;
use pdf_115::file::File;
use libfuzzer_sys::arbitrary::{self, Arbitrary};

fuzz_target!(|data: &str| {
    pdf_115::file::File::from_data(data.as_ref());
});
