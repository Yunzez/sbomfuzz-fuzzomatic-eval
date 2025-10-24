#![no_main]

use libfuzzer_sys::fuzz_target;
use symbolic::unreal::Unreal4Crash;

fuzz_target!(|data: &[u8]| {
    // Use a fixed limit for fuzzing. You may vary this as required.
    let limit = 1024 * 1024;

    // Attempt to parse the input as an Unreal4Crash object
    let _ = Unreal4Crash::parse_with_limit(data, limit);
});