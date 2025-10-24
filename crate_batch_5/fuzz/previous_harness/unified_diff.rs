#![no_main]

extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use arbitrary::{Arbitrary, Unstructured};
use unified_diff::diff;

#[derive(Debug, Arbitrary)]
struct DiffInput {
    expected: Vec<u8>,
    actual: Vec<u8>,
    context_size: usize,
}

// Arbitrary implementation for filename validity.
fn is_valid_filename(filename: &str) -> bool {
    !filename.bytes().any(|b| b < b' ')
}

fuzz_target!(|data: DiffInput| {
    let expected_filename = "a/fuzz.file";
    let actual_filename = "target/fuzz.file";

    if let Ok(expected_str) = String::from_utf8(data.expected.clone()) {
        if !expected_str.is_ascii() || expected_str.find(|x| x < ' ' && x != '\n').is_some() {
            return;
        }
    } else {
        return;
    }

    if let Ok(actual_str) = String::from_utf8(data.actual.clone()) {
        if !actual_str.is_ascii() || actual_str.find(|x| x < ' ' && x != '\n').is_some() {
            return;
        }
    } else {
        return;
    }

    if is_valid_filename(expected_filename) && is_valid_filename(actual_filename) {
        let _ = diff(
            &data.expected,
            expected_filename,
            &data.actual,
            actual_filename,
            data.context_size
        );
    }
});