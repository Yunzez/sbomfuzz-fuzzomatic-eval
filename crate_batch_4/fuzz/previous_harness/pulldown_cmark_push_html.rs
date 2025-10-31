#![no_main]
use libfuzzer_sys::fuzz_target;
use pulldown_cmark_131::{html, Parser};

fuzz_target!(|input: (&str, &str)| {
    let (text, output) = input;
    let parser = Parser::new(text);

    let mut output_str = output.to_string();
    html::push_html(&mut output_str, parser);
});