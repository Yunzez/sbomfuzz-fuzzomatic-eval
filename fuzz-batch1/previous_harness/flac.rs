#![no_main]
use libfuzzer_sys::fuzz_target;
use flac::{ByteStream, Stream};

fuzz_target!(|data: &[u8]| {
    let _ = Stream::<ByteStream>::from_buffer(data);
});
