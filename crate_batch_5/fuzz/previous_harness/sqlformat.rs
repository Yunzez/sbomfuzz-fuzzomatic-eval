#![no_main]

use libfuzzer_sys::fuzz_target;
use sqlformat::{format, QueryParams, FormatOptions as  SqlFormatOptions};
use arbitrary::Arbitrary;
use sqlformat::*;
#[derive(Arbitrary, Debug)]
struct FuzzInput {
    query: String,
}

fuzz_target!(|input: FuzzInput| {
    let _ = std::panic::catch_unwind(|| {
        format(&input.query, &QueryParams::None, SqlFormatOptions::default());
    });
});