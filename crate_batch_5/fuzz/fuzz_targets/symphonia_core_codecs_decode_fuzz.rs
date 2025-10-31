#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::io::Cursor;
use std::panic;
use symphonia::core::{errors, formats};
use symphonia::default::get_codecs;
use symphonia::core::codecs::{ CodecParameters, DecoderOptions };
use symphonia::core::formats::{ FormatOptions, FormatReader };
use symphonia::core::probe::Hint;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_probe;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let FuzzInput { data } = input;

    // Set up the media source stream
    let data_cursor = Cursor::new(data);
    let mss = MediaSourceStream::new(Box::new(data_cursor), Default::default());

    // Try probing the format
    if let Ok(probed) = get_probe().format(
        &Hint::new(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default()
    ) {
        let mut reader = probed.format;

        if let Some(track) = reader.default_track() {
            let params = track.codec_params.clone();

            // Attempt to make a decoder and decode
            if let Ok(mut decoder) = panic::catch_unwind(|| {
                get_codecs().make(&params, &DecoderOptions::default())
            }).and_then(|res| Ok(res)) {
                if let Ok(mut decoder) = decoder {
                    while let Ok(packet) = reader.next_packet() {
                        let _ = decoder.decode(&packet);
                    }
                }
            }
        }
    }
});