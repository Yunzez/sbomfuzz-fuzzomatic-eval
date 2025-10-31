#![no_main]

use libfuzzer_sys::fuzz_target;
use syn_188::{parse_str, Expr};

fuzz_target!(|data: &str| {
    let _ = std::panic::catch_unwind(|| {
        let _ = parse_str::<Expr>(data);
    });
});