#![no_main]

use libfuzzer_sys::fuzz_target;
use ssh_parser::ssh::parse_ssh_packet;

fuzz_target!(|data: &[u8]| {
    let _ = std::panic::catch_unwind(|| {
        let _ = parse_ssh_packet(data);
    });
});