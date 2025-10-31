#![no_main]
use libfuzzer_sys::fuzz_target;
use chrono_17::{DateTime, Utc};
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    days: u64,
}

fuzz_target!(|data: FuzzInput| {
    let dt = Utc::now();
    let days = chrono_17::Days::new(data.days);
    let _ = dt.checked_add_days(days);
});