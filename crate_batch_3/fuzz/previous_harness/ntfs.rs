#![no_main]
extern crate arbitrary;
extern crate libfuzzer_sys;
use std::io::Cursor;
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

// #[derive(Debug)]
// struct NTFSFuzzInput {
//     data: Vec<u8>,
// }

// impl<'a> Arbitrary<'a> for NTFSFuzzInput {
//     fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
//         let data = u.arbitrary::<Vec<u8>>()?;
//         Ok(NTFSFuzzInput { data })
//     }
// }


fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data);
    let _ = ntfs::Ntfs::new(&mut cursor);
});