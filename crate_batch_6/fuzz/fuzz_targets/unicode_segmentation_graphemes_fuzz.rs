#![no_main]
use libfuzzer_sys::fuzz_target;
use unicode_segmentation::UnicodeSegmentation;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    s: &'a str,
}

fuzz_target!(|data: FuzzInput| {
    let forward = UnicodeSegmentation::graphemes(data.s, true).collect::<Vec<_>>();
    let forward_reversed: Vec<_> = forward.clone().into_iter().rev().collect();
    let reverse = UnicodeSegmentation::graphemes(data.s, true).rev().collect::<Vec<_>>();
    assert_eq!(forward_reversed, reverse);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: unicode_segmentation 
//  - Crate Link: https://github.com/unicode-rs/unicode-segmentation 
//  - Crate Version: 1.1.0 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: unicode_segmentation 
//  - Use Statement: use unicode_segmentation::UnicodeSegmentation; 
//  - Function Name: graphemes 
//  - Function Usage: fn run_7() {
//     println!("run 7");
// 
//     // ? line 211
//     let s = "\u{1F938}\u{1F3FE}\u{1F3FE}";
//     let forward = UnicodeSegmentation::graphemes(s, true).collect::<Vec<_>>();
//     let forward_reversed = forward.into_iter().rev().collect::<Vec<_>>();
//     let reverse = UnicodeSegmentation::graphemes(s, true).rev().collect::<Vec<_>>();
//     assert_eq!(forward_reversed, reverse);
// 
//     // ? line 212
//     let s =
//         "j\u{FFFD}jjjjjjjjjjj\u{0489}\u{200D}\u{2764}jjjjjjjjj\u{0489}j\u{FFFD}\u{FFFD}\u{FFFD}\"jjjjjj\"jjD\u{0409}\u{0489}0\\f\u{FFFD}";
//     let forward = s.split_word_bounds().collect::<Vec<_>>();
//     let forward_reversed = forward.into_iter().rev().collect::<Vec<_>>();
//     let reverse = s.split_word_bounds().rev().collect::<Vec<_>>();
//     // ? assert would fail, that's the bug
//     assert_eq!(forward_reversed, reverse);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a Self, bool) -> Graphemes<'a>,
//   type_fields: [Self, unicode_segmentation::Graphemes] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a Self, bool) -> Graphemes<'a>,
//   type_fields: [Self, unicode_segmentation::Graphemes] 
// }
// Struct construction metadata: {
//   type_name: unicode_segmentation::Graphemes,
//   type_fields: [&'a str, unicode_segmentation::GraphemeCursor, unicode_segmentation::GraphemeCursor] 
// }
// Struct construction metadata: {
//   type_name: unicode_segmentation::GraphemeCursor,
//   type_fields: [usize, usize, bool, unicode_segmentation::GraphemeState, core::Option, core::Option, core::Option, core::Option, bool] 
// }
// Struct construction metadata: {
//   type_name: unicode_segmentation::GraphemeState,
//   type_fields: [Unknown, NotBreak, Break, CheckCrlf, Regional, Emoji] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }

