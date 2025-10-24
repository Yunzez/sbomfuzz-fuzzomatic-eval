#![no_main]
use libfuzzer_sys::fuzz_target;
use fancy_regex::Regex;
use arbitrary::Arbitrary;

fuzz_target!(|data: &str| {
    // Try to safely create a new Regex using the input data
    if let Ok(_) = Regex::new(data) {
        // Successfully created a Regex, proceed to test it with various inputs if needed
    }
});
