#![no_main]

extern crate libfuzzer_sys;

use crate_batch_4;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(input) = std::str::from_utf8(data) {
        // Attempt to parse the input into the expected type
        let parsed_data: Vec<Vec<(String, u8)>> = input
            .lines()
            .map(|line| {
                line.split(',')
                    .filter_map(|pair| {
                        let mut parts = pair.split(':');
                        if let (Some(s), Some(n)) = (parts.next(), parts.next()) {
                            if let Ok(num) = n.parse::<u8>() {
                                return Some((s.to_string(), num));
                            }
                        }
                        None
                    })
                    .collect()
            })
            .collect();

        crate_batch_4::run_9(parsed_data);
    }
});
