#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use png::{Decoder, Limits};
use std::io::Cursor;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let limits = Limits { bytes: 1 << 16 };
    let cursor = Cursor::new(&input.data);
    let decoder = Decoder::new_with_limits(cursor, limits);

    if let Ok((info, mut reader)) = decoder.read_info() {
        if info.buffer_size() <= 5_000_000 {
            let mut buffer = vec![0u8; info.buffer_size()];
            let _ = reader.next_frame(&mut buffer);
        }
    }
});