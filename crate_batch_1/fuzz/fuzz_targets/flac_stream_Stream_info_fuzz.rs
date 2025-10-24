#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use flac::stream::Stream;
use flac::ByteStream;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    buffer: Vec<u8>,
}

fuzz_target!(|data: FuzzInput| {
    let s = Stream::<ByteStream>::from_buffer(&data.buffer);
    if let Ok(mut stream) = s {
        let _ = stream.info();
    }
});