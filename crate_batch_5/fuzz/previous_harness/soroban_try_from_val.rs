#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_env_common::Symbol;
use arbitrary::Arbitrary;
use soroban_env_host::Host;
use soroban_env_common::TryFromVal;
fuzz_target!(|data: &str| {
    let host = Host::default();
    let _ = Symbol::try_from_val(&host, &data);
});
