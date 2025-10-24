#![no_main]
use libfuzzer_sys::fuzz_target;
use png::{OutputInfo, ColorType, BitDepth};

#[derive(Arbitrary, Debug)]
struct FuzzOutputInfo {
    width: u32,
    height: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
    line_size: usize,
}

fuzz_target!(|info: FuzzOutputInfo| {
    let output_info = OutputInfo {
        width: info.width,
        height: info.height,
        color_type: info.color_type,
        bit_depth: info.bit_depth,
        line_size: info.line_size,
    };

    // Using the buffer_size method from OutputInfo
    let _ = output_info.buffer_size();
});