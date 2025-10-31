#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use pulldown_cmark_128::Options; // Adjusted to the suggested module

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    option_value: u32,
}

fuzz_target!(|data: FuzzInput| {
    let mut opts = Options::empty();
    if let Some(option) = Options::from_bits(data.option_value) {
        opts.insert(option);
    }
});