#![no_main]
use libfuzzer_sys::fuzz_target;
use chrono_16::DateTime;
use chrono_16::FixedOffset;
use chrono_16::ParseError;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    email_date_time: String,
}

fuzz_target!(|data: FuzzInput| {
    let _ = DateTime::parse_from_rfc2822(&data.email_date_time);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: chrono 
//  - Crate Link: https://github.com/chronotope/chrono 
//  - Crate Version: 0.5.0-alpha.1 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: chrono::datetime::(Struct)DateTime 
//  - Use Statement: None 
//  - Function Name: checked_add_days 
//  - Function Usage: fn run_6() {
//     print!("chrono_16");
//     chrono_16::DateTime::parse_from_rfc2822("31 DEC 262143 23:59 -2359");
//     chrono_17::DateTime::checked_add_days(chrono_17::Utc::now(), chrono_17::Days::new(1));
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(DateTime<Tz>, Days) -> Option<DateTime<Tz>>,
//   type_fields: [Tz, chrono::Days] 
// }
// Struct construction metadata: {
//   type_name: fn(DateTime<Tz>, Days) -> Option<DateTime<Tz>>,
//   type_fields: [Tz, chrono::Days] 
// }
// Struct construction metadata: {
//   type_name: chrono::Days,
//   type_fields: [u64] 
// }

