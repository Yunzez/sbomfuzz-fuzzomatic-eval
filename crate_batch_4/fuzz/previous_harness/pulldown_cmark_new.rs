#![no_main]
use libfuzzer_sys::fuzz_target;
use pulldown_cmark_128::Parser;

fuzz_target!(|data: &str| {
    let _ = Parser::new(data);
});