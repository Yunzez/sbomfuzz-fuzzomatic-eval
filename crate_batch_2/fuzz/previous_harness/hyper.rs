#![no_main]
use libfuzzer_sys::fuzz_target;
use hyper::{ Uri, Url };
use arbitrary::Arbitrary;

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    url: String
}

fuzz_target!(|data: FuzzInput| {
    if let Ok(parsed_url) = Url::parse(&data.url) {
        let _ = Uri::from(parsed_url);
    }
});