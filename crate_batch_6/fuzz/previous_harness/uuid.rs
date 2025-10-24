#![no_main]

use libfuzzer_sys::fuzz_target;
use uuid::Uuid;

fuzz_target!(|data: &str| {
    let _ = Uuid::parse_str(data);
});