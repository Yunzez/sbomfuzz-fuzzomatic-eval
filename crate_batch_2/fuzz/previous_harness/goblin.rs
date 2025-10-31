#![no_main]
use libfuzzer_sys::fuzz_target;
use goblin::archive::Index;

fuzz_target!(|data: &[u8]| {
    let _ = Index::parse_sysv_index(data);
});