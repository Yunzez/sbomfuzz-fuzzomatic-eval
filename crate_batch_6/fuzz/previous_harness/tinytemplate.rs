#![no_main]

use libfuzzer_sys::fuzz_target;
use tinytemplate::TinyTemplate;

fuzz_target!(|data: (String, String)| {
    let (name, text) = data;
    let mut tpl = TinyTemplate::new();
    let _ = tpl.add_template(&name, &text);
});