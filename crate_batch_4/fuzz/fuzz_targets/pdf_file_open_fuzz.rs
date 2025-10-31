#![no_main]
use libfuzzer_sys::fuzz_target;
use pdf_115::file::File;
use arbitrary::Arbitrary;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    path_data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let os_str = OsStr::from_bytes(&input.path_data);
    let path = Path::new(os_str);
    let _result = File::open(path);
});