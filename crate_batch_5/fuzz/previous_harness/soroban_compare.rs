#![no_main]
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary::Arbitrary;

use std::convert::TryFrom;
use soroban_env_host::budget::{ Budget };
use std::io::Cursor;
use soroban_env_host::{ Compare, Host };
use soroban_env_common::xdr::{ScVec, ScVal};

fuzz_target!(|input: (Vec<u32>, Vec<u32>)| {
    let (a_data, b_data) = input;
    let a_scval: Vec<ScVal> = a_data.into_iter().map(ScVal::U32).collect();
    let b_scval: Vec<ScVal> = b_data.into_iter().map(ScVal::U32).collect();
    // Attempt to construct ScVec from the fuzz data
    if let (Ok(v1), Ok(v2)) = (ScVec::try_from(a_scval), ScVec::try_from(b_scval)) {
        let budget = Budget::default();
        
        // Attempt to compare using the budget's compare function
        let _ = budget.compare(&v1, &v2);
    }
});