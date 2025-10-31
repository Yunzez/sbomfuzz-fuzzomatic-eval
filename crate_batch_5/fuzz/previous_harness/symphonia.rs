#![no_main]

use libfuzzer_sys::fuzz_target;
use symphonia::core::codecs::{ CodecParameters, DecoderOptions };
use symphonia::core::errors;
use symphonia::core::formats::{ FormatOptions, FormatReader };
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::core::meta::MetadataOptions;
use symphonia::default::get_probe;
use std::io::Cursor;
use std::sync::Arc;

fuzz_target!(|data: &[u8]| {
    let _ = std::panic::catch_unwind(|| {
        let arc_data: Arc<[u8]> = data.to_vec().into(); // now has 'static lifetime
        let cursor = Cursor::new(arc_data.clone()); // Cursor holds Arc

        let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

        let probed = get_probe().format(
            &Hint::new(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default()
        );

        if let Ok(probed) = probed {
            let reader = probed.format;

            if let Some(track) = reader.default_track() {
                let params = track.codec_params.clone();

                let _ = std::panic::catch_unwind(|| {
                    symphonia::default::get_codecs().make(&params, &DecoderOptions::default())
                });
            }
        }
    });
});
