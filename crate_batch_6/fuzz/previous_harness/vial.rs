#![no_main]

use std::io::{self, Cursor};
use libfuzzer_sys::fuzz_target;
use vial::Request;

fuzz_target!(|data: &[u8]| {
    let mut reader = Cursor::new(data);
    if let Ok(request) = Request::from_reader(&mut reader) {
        let _ = vial::util::percent_decode(request.path());
    }
});