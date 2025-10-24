#![no_main]
extern crate libfuzzer_sys;
extern crate png;
use libfuzzer_sys::fuzz_target;
use std::io::Cursor;
use png::Decoder;
use png::Limits;
use std::io;
fn decode_png(data: &[u8])
    -> io::Result<Vec<u8>>
{
    let limits = Limits { bytes: 1 << 16 };
    let decoder = Decoder::new_with_limits(data, limits);
    let (info, mut reader) = match decoder.read_info() {
        Ok(result) => result,
        Err(e) => return Err(e.into()),
    };

    if info.buffer_size() > 5_000_000 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "memory exceeded"));
    }
    let mut buffer = vec![0u8; info.buffer_size()];
    let _ = reader.next_frame(&mut buffer);
    Ok(buffer)
}

fuzz_target!(|data: &[u8]| {
    decode_png(data);
});
