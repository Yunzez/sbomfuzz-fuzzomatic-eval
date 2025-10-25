#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate libfuzzer_sys;
extern crate cssparser;
extern crate arbitrary;

use cssparser::Parser;
use cssparser::ParserInput;
use arbitrary::Arbitrary;
use arbitrary::Unstructured;

fuzz_target!(|data: &[u8]| {
    if let Ok(input_string) = std::str::from_utf8(data) {
        let mut parser_input = ParserInput::new(input_string);
        let mut parser = Parser::new(&mut parser_input);

        // Handle the result
        let _ = parser.next_including_whitespace_and_comments();
    }
});