#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use pulldown_cmark_128::Parser;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a str,
}

fuzz_target!(|input: FuzzInput| {
    let _parser = Parser::new(input.data);
    // The parser output is not used here but can be further processed if needed
});