#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use flac::stream::Stream;
use flac::ByteStream;

fuzz_target!(|data: Vec<u8>| {
    let _ = Stream::<ByteStream>::from_buffer(&data);
});