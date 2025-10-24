#![no_main]
use libfuzzer_sys::fuzz_target;
use phonenumber::parse;
use arbitrary::Arbitrary;
use std::str::FromStr;
#[derive(Debug, Arbitrary)]
struct InputData {
    country: Option<String>,
    number: String,
}

fuzz_target!(|data: InputData| {
    let country = match &data.country {
        Some(s) => phonenumber::country::Id::from_str(s).ok(),
        None => None,
    };
    // Assuming parse returns a Result that we can safely unwrap or handle
    let _ = phonenumber::parse(country, &data.number);
});