#![no_main]

use libfuzzer_sys::fuzz_target;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    sql: String,
}

fuzz_target!(|data: FuzzInput| {
    let dialect = GenericDialect {};

    let _ = std::panic::catch_unwind(|| {
        Parser::parse_sql(&dialect, &data.sql).ok(); // Using ok() to discard result and focus on crash detection
    });
});