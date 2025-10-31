#![no_main]

extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use ssh_keys::openssh::parse_private_key;

fuzz_target!(|data: &str| {
    // Using catch_unwind to catch any potential panics during fuzzing
    let _ = std::panic::catch_unwind(|| {
        parse_private_key(data);
    });
});