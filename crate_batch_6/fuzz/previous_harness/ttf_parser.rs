
#![no_main]
use libfuzzer_sys::fuzz_target;
use ttf_parser::{Face, GlyphId, OutlineBuilder};

struct Builder;

impl OutlineBuilder for Builder {
    fn move_to(&mut self, _: f32, _: f32) {}
    fn line_to(&mut self, _: f32, _: f32) {}
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {}
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {}
    fn close(&mut self) {}
}

fuzz_target!(|data: &[u8]| {
    if let Ok(face) = Face::from_slice(data, 0) {
        let _ = face.outline_glyph(GlyphId(0), &mut Builder);
    }
});