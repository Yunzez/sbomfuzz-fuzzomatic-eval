#![no_main]
use libfuzzer_sys::fuzz_target;
use handlebars::Handlebars;
use std::str;

fuzz_target!(|input: (&[u8], &[u8])| {
    if let (Ok(template_string), Ok(data_string)) = (str::from_utf8(input.0), str::from_utf8(input.1)) {
        let data = Vec::<()>::new(); // Simulate a data source for testing
        let handlebars = Handlebars::new();
        let _ = handlebars.render_template(template_string, &data_string); // This uses &str for data just for fuzzing purposes
    }
});