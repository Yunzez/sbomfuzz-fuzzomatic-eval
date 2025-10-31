#![no_main]

use libfuzzer_sys::fuzz_target;
use semver::VersionReq;

fuzz_target!(|data: &[u8]| {
    if let Ok(input_str) = std::str::from_utf8(data) {
        let _ = VersionReq::parse(input_str);
    }
});