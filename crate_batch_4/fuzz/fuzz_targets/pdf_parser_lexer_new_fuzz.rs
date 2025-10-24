#![no_main]
use libfuzzer_sys::fuzz_target;
use pdf_115::parser::Lexer;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a [u8],
}

fuzz_target!(|input: FuzzInput| {
    let _lexer = Lexer::new(input.data);
});