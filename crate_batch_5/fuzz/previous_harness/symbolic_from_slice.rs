#![no_main]

use libfuzzer_sys::fuzz_target;
use symbolic::common::ByteView;

fuzz_target!(|data: &[u8]| {
    let _ = ByteView::from_slice(data);
});