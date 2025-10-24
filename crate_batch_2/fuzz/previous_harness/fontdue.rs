#![no_main]
use libfuzzer_sys::fuzz_target;
use fontdue::{Font, FontSettings};

fuzz_target!(|data: &[u8]| {
    // let _ = Font::from_bytes(data, FontSettings::default());
    if let Ok(font) = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()) {
        let (metrics, bitmap) = font.rasterize('g', 17.0);
    }
});