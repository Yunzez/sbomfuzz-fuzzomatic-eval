#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    sql: String,
}

fuzz_target!(|data: FuzzInput| {
    let dialect = GenericDialect {};

    let _ = std::panic::catch_unwind(|| {
        let _ = Parser::parse_sql(&dialect, &data.sql);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: sqlparser 
//  - Crate Link: https://github.com/ballista-compute/sqlparser-rs 
//  - Crate Version: 0.9.1-alpha.0 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: sqlparser::parser::(Struct)Parser 
//  - Use Statement: use sqlparser::parser::Parser; 
//  - Function Name: parse_sql 
//  - Function Usage: fn run_12() {
//     println!("run 12");
//     // ! crashing input
//     // let sql = "(".repeat(1024);
//     let sql = "(".repeat(1);
//     let dialect = GenericDialect {};
// 
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             Parser::parse_sql(&dialect, &sql);
//         })
//     {
//         eprintln!("Caught panic in run 12: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&(dyn Dialect + 'static), &str) -> Result<Vec<Statement, Global>, ParserError>,
//   type_fields: [dyn Dialect + 'static, sqlparser::Statement, alloc::Global, sqlparser::ParserError] 
// }
// Struct construction metadata: {
//   type_name: fn(&(dyn Dialect + 'static), &str) -> Result<Vec<Statement, Global>, ParserError>,
//   type_fields: [dyn Dialect + 'static, sqlparser::Statement, alloc::Global, sqlparser::ParserError] 
// }
// Struct construction metadata: {
//   type_name: sqlparser::Statement,
//   type_fields: [Analyze, Truncate, Msck, Query(Box<Query>), Insert, Directory, Copy, Update, Delete, CreateView, CreateTable, CreateVirtualTable, CreateIndex, AlterTable, Drop, SetVariable, ShowVariable, ShowColumns, StartTransaction, SetTransaction, Commit, Rollback, CreateSchema, CreateDatabase, Assert, Deallocate, Execute, Prepare, Explain] 
// }
// Struct construction metadata: {
//   type_name: sqlparser::ParserError,
//   type_fields: [TokenizerError(String), ParserError(String)] 
// }

