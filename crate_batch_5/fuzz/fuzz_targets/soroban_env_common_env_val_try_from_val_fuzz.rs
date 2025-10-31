#![no_main]
use libfuzzer_sys::fuzz_target;
use soroban_env_common::Symbol;
use soroban_env_common::{Env, TryFromVal};
use soroban_env_host::budget::{ Budget };
use soroban_env_host::{ Compare, Host };
use arbitrary::Arbitrary;
use std::convert::TryFrom;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    input_str: String,
}

fuzz_target!(|data: FuzzInput| {
    let host = Host::default();
    let _ = Symbol::try_from_val(&host, &data.input_str.as_str());
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
//  - Module Path: soroban_env_common::env_val 
//  - Use Statement: use soroban_env_common::Symbol 
//  - Function Name: try_from_val 
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
//   type_name: fn(&E, &V) -> Result<Self, <Self as TryFromVal<E, V>>::Error>,
//   type_fields: [E, V, Self] 
// }
// Struct construction metadata: {
//   type_name: fn(&E, &V) -> Result<Self, <Self as TryFromVal<E, V>>::Error>,
//   type_fields: [E, V, Self] 
// }

