#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use csscolorparser::parse;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    input: &'a str,
}

fuzz_target!(|data: FuzzInput| {
    let _ = parse(data.input);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: csscolorparser 
//  - Crate Link: None 
//  - Crate Version: 0.6.0 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: csscolorparser::parser 
//  - Use Statement: None 
//  - Function Name: parse 
//  - Function Usage: fn run_9() {
//     // ! parsehex is private, parse gets to parsehex
//     csscolorparser::parse("#123456");
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Result<Color, ParseColorError>,
//   type_fields: [csscolorparser::Color, csscolorparser::ParseColorError] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Result<Color, ParseColorError>,
//   type_fields: [csscolorparser::Color, csscolorparser::ParseColorError] 
// }
// Struct construction metadata: {
//   type_name: csscolorparser::Color,
//   type_fields: [f64, f64, f64, f64] 
// }
// Struct construction metadata: {
//   type_name: csscolorparser::ParseColorError,
//   type_fields: [InvalidHex, InvalidRgb, InvalidHsl, InvalidHwb, InvalidHsv, InvalidFunction, InvalidUnknown] 
// }

