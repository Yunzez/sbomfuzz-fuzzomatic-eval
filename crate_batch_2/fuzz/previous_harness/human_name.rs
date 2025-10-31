#![no_main]
extern crate libfuzzer_sys;
extern crate human_name;
use libfuzzer_sys::fuzz_target;
use human_name::Name;

fuzz_target!(|data: &str| {
    let _ = Name::parse(data);
});