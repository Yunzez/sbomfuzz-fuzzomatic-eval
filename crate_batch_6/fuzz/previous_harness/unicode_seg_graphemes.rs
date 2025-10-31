#![no_main]
use libfuzzer_sys::fuzz_target;
use unicode_segmentation::UnicodeSegmentation;

fuzz_target!(|data: (&str, bool)| {
    let (s, is_extended) = data;

    // Test grapheme segmentation with forward and reverse comparison
    let forward = UnicodeSegmentation::graphemes(s, is_extended).collect::<Vec<_>>();
    let forward_reversed = forward.clone().into_iter().rev().collect::<Vec<_>>();
    let reverse = UnicodeSegmentation::graphemes(s, is_extended).rev().collect::<Vec<_>>();
    assert_eq!(forward_reversed, reverse);

    // Test word boundary segmentation with forward and reverse comparison
    let forward = s.split_word_bounds().collect::<Vec<_>>();
    let forward_reversed = forward.clone().into_iter().rev().collect::<Vec<_>>();
    let reverse = s.split_word_bounds().rev().collect::<Vec<_>>();
    // Note: This assertion might fail if there's a bug in the library
    assert_eq!(forward_reversed, reverse);
});