#![no_main]

use libfuzzer_sys::fuzz_target;
use vobsub::subtitles;

fuzz_target!(|data: &[u8]| {
    for _ in vobsub::subtitles(data) {
        // Just parse and ignore.
    }
});