#![no_main]

use libfuzzer_sys::fuzz_target;
use unicode_segmentation::UnicodeSegmentation;

fuzz_target!(|data: &str| {
    let _ = data.split_word_bounds().collect::<Vec<_>>();
});