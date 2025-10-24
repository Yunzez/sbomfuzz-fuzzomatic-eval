#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use gif::{DecodeOptions, ColorOutput};
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let mut options = DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let cursor = Cursor::new(data);
    if let Ok(mut decoder) = options.read_info(data) {
        while let Ok(Some(_frame)) = decoder.read_next_frame() {}
    }
});
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: gif 
//  - Crate Link: https://github.com/image-rs/image-gif 
//  - Crate Version: 0.11.3 
//  - From Crate: crate_batch_2 
//  - From Crate Link: unknown_website 
//  - Module Path: gif::reader::(Struct)DecodeOptions 
//  - Use Statement: None 
//  - Function Name: set_color_output 
//  - Function Usage: fn run_10() {
//     let mut options = gif::DecodeOptions::new();
// 
//     options.set_color_output(gif::ColorOutput::RGBA);
//     let data = std::io::Cursor::new(
//         &[
//             0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0xff, 0x00, 0xff,
//             0xff, 0xff, 0x00, 0x00, 0x00, 0x21, 0xf9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2c,
//             0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x02, 0x44, 0x01, 0x00,
//             0x3b,
//         ]
//     );
//     if let Ok(mut decoder) = options.read_info(data) {
//         while let Ok(Some(_frame)) = decoder.read_next_frame() {}
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut DecodeOptions, ColorOutput),
//   type_fields: [gif::DecodeOptions, gif::ColorOutput] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut DecodeOptions, ColorOutput),
//   type_fields: [gif::DecodeOptions, gif::ColorOutput] 
// }
// Struct construction metadata: {
//   type_name: gif::DecodeOptions,
//   type_fields: [gif::MemoryLimit, gif::ColorOutput, bool, bool, bool] 
// }
// Struct construction metadata: {
//   type_name: gif::MemoryLimit,
//   type_fields: [u32] 
// }
// Struct construction metadata: {
//   type_name: gif::ColorOutput,
//   type_fields: [RGBA, Indexed] 
// }

