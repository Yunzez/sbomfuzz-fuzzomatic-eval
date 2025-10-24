#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use pgp::types::Version;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: &[u8]| {
    let version = Version::New;
    // Using provided Vec<u8> from FuzzInput
    let _ = pgp::Signature::from_slice(version, &input);
});