#![no_main]
extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;
use chrono_17::{DateTime, Utc};
use chrono_17::Days;
use chrono_17::TimeZone;
fuzz_target!(|input: (i64, u64)| {
    // Destructuring the tuple
    let (timestamp, num_days) = input;
    
    // Creating a DateTime from a fixed timestamp
    chrono_17::DateTime::checked_add_days(chrono_17::Utc::now(), chrono_17::Days::new(num_days));
    
});