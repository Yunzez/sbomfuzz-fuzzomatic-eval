#![no_main]
use libfuzzer_sys::fuzz_target;
use serde_yaml::to_string;
use serde::{Serialize, Deserialize};
use arbitrary::Arbitrary;
#[derive(Serialize, Deserialize, Debug, PartialEq, Arbitrary)]
enum FuzzValue {
    Integer(i64),
    Bool(bool),
    String(String),
    Vec(Vec<FuzzValue>),
}

fuzz_target!(|value: FuzzValue| {
    let _ = to_string(&value);
});