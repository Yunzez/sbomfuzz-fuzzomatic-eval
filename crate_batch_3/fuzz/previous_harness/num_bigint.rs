#![no_main]
extern crate libfuzzer_sys;
use libfuzzer_sys::fuzz_target;
use num_bigint::BigUint;
use arbitrary::Arbitrary;
use std::str::FromStr;
use num_traits::Num;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    s: &'a str,
    radix: u32,
}
// 
fuzz_target!(|data: FuzzInput| {
    if let Ok(radix) = u32::try_from(data.radix) {
        let _ = BigUint::from_str_radix(data.s, radix).unwrap_or_default();
    }
});