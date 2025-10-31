#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str::FromStr;
use todotxt::Task;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        // Attempt to parse string as a Task
        let _: Result<Task, _> = Task::from_str(s);
    }
});
