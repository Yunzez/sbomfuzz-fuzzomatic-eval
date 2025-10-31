#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use sqlformat::{format, QueryParams, FormatOptions as  SqlFormatOptions, Indent};

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    sql: &'a str,
    query_params: QueryParamsData,
    indent_type: IndentType,
    uppercase: bool,
    keyword_case: u8,
}

#[derive(Arbitrary, Debug)]
enum QueryParamsData {
    Named(Vec<(String, String)>),
    Indexed(Vec<String>),
    None,
}

#[derive(Arbitrary, Debug)]
enum IndentType {
    Spaces(u8),
    Tabs,
}

impl<'a> From<FuzzInput<'a>> for (String, QueryParams, SqlFormatOptions) {
    fn from(input: FuzzInput<'a>) -> Self {
        let query_params = match input.query_params {
            QueryParamsData::Named(pairs) => QueryParams::Named(pairs),
            QueryParamsData::Indexed(indexes) => QueryParams::Indexed(indexes),
            QueryParamsData::None => QueryParams::None,
        };

        let indent = match input.indent_type {
            IndentType::Spaces(spaces) => Indent::Spaces(spaces),
            IndentType::Tabs => Indent::Tabs,
        };

        let format_options = SqlFormatOptions {
            indent,
            uppercase: input.uppercase,
            lines_between_queries: 1,
        };

        (input.sql.to_string(), query_params, format_options)
    }
}

fuzz_target!(|data: FuzzInput| {
    let (sql, query_params, format_options) = data.into();
    let _ = format(&sql, &query_params, format_options);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: sqlformat 
//  - Crate Link: https://github.com/shssoichiro/sqlformat-rs 
//  - Crate Version: 0.1.8 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: sqlformat 
//  - Use Statement: None 
//  - Function Name: format 
//  - Function Usage: fn run_20() {
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             let _ = format(
//                 "?62666666121266666612",
//                 &QueryParams::None,
//                 SqlFormatOptions::default()
//             );
//         })
//     {
//         eprintln!("Caught panic in run 20: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str, &QueryParams, FormatOptions) -> String,
//   type_fields: [sqlformat::QueryParams, sqlformat::FormatOptions, alloc::String] 
// }
// Struct construction metadata: {
//   type_name: fn(&str, &QueryParams, FormatOptions) -> String,
//   type_fields: [sqlformat::QueryParams, sqlformat::FormatOptions, alloc::String] 
// }
// Struct construction metadata: {
//   type_name: sqlformat::QueryParams,
//   type_fields: [Named(Vec<(String, String)>), Indexed(Vec<String>), None] 
// }
// Struct construction metadata: {
//   type_name: sqlformat::FormatOptions,
//   type_fields: [sqlformat::Indent, bool, u8] 
// }
// Struct construction metadata: {
//   type_name: sqlformat::Indent,
//   type_fields: [Spaces(u8), Tabs] 
// }
// Struct construction metadata: {
//   type_name: alloc::String,
//   type_fields: [alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [alloc::RawVec, usize] 
// }
// Struct construction metadata: {
//   type_name: alloc::RawVec,
//   type_fields: [alloc::RawVecInner, core::PhantomData] 
// }
// Struct construction metadata: {
//   type_name: alloc::RawVecInner,
//   type_fields: [core::Unique, core::UsizeNoHighBit, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: core::Unique,
//   type_fields: [core::NonNull, core::PhantomData] 
// }
// Struct construction metadata: {
//   type_name: core::NonNull,
//   type_fields: [*const u8] 
// }
// Struct construction metadata: {
//   type_name: core::UsizeNoHighBit,
//   type_fields: [usize] 
// }

