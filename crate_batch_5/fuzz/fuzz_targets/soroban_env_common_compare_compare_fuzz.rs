#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use soroban_env_host::{ Compare, Host };
use soroban_env_common::xdr::{ScVec, ScVal};
use soroban_env_host::budget::{ Budget };
use std::cmp::Ordering;
use std::convert::TryFrom;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    tuple1: (u32, u32),
    tuple2: (u32, u32, u32),
}

fuzz_target!(|data: FuzzInput| {
    if let (Ok(v1), Ok(v2)) = (
        ScVec::try_from(data.tuple1),
        ScVec::try_from(data.tuple2),
    ) {
        let budget = Budget::default();
        let _ = budget.compare(&v1, &v2);
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: soroban_env_common 
//  - Crate Link: https://github.com/stellar/rs-soroban-env 
//  - Crate Version: 0.0.15 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: soroban_env_common::compare 
//  - Use Statement: None 
//  - Function Name: compare 
//  - Function Usage: fn run_11() {
//     println!("run 11");
//     // ? line 172
//     let v1 = ScVec::try_from((0, 1)).unwrap();
//     let v2 = ScVec::try_from((0, 0, 2)).unwrap();
//     let expected_cmp = Ordering::Greater;
//     let budget = Budget::default();
//     let actual_cmp = budget.compare(&v1, &v2).unwrap();
// 
//     // ? line 174
//     let host = Host::default();
//     let s = "#";
//     let s = Symbol::try_from_val(&host, &s);
//     match s {
//         Ok(_) => println!("Unexpected success in symbol conversion"),
//         Err(_) => println!("Symbol conversion failed as expected"),
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&Self, &T, &T) -> Result<Ordering, <Self as Compare<T>>::Error>,
//   type_fields: [Self, T, core::Ordering] 
// }
// Struct construction metadata: {
//   type_name: fn(&Self, &T, &T) -> Result<Ordering, <Self as Compare<T>>::Error>,
//   type_fields: [Self, T, core::Ordering] 
// }
// Struct construction metadata: {
//   type_name: core::Ordering,
//   type_fields: [Less, Equal, Greater] 
// }

