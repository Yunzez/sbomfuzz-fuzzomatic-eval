#![no_main]

use libfuzzer_sys::fuzz_target;
use swf_parser::streaming::movie;

fuzz_target!(|data: &[u8]| {
    // Call the function with the fuzzed input
    let _ = std::panic::catch_unwind(|| {
        let _ = movie::parse_movie(data);
    });
});