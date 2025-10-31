#![no_main]

extern crate libfuzzer_sys;
extern crate url;

use libfuzzer_sys::fuzz_target;
use url::Url;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let _ = Url::parse(input);
    }
});