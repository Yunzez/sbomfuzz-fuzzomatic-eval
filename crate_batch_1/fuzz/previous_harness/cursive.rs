#![no_main]
use libfuzzer_sys::fuzz_target;
use cursive::{Cursive, views::Dialog, views::TextView};
use cursive::CursiveExt;
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut siv = Cursive::default();
        siv.add_layer(
            Dialog::around(TextView::new(s))
                .title("Cursive")
                .button("Quit", |s| s.quit()),
        );
        siv.run();
    }
});