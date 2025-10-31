#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use pulldown_cmark_128::Parser;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    input_str: &'a str,
}

fuzz_target!(|data: FuzzInput| {
    let parser = Parser::new(data.input_str);
    
    // Attempt to process the parser to ensure it doesn't crash
    for _ in parser {}
});