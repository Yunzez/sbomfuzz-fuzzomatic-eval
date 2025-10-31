#![no_main]
use libfuzzer_sys::fuzz_target;
use tokei::{Config, Language, LanguageType};
use arbitrary::Arbitrary;
use std::panic;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    language: LanguageType,
    text: &'a [u8],
    treat_doc_strings_as_comments: Option<bool>,
}

fuzz_target!(|data: FuzzInput| {
    let config = Config {
        treat_doc_strings_as_comments: data.treat_doc_strings_as_comments,
        ..Config::default()
    };
    
    let _ = panic::catch_unwind(|| {
        data.language.parse_from_slice(data.text, &config);
    });
});