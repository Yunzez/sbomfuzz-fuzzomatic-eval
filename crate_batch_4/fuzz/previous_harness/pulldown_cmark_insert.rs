#![no_main]
use libfuzzer_sys::fuzz_target;
use pulldown_cmark_133::Options;
use arbitrary::{Arbitrary, Unstructured};

#[derive(Arbitrary)]
struct Opts {
    enable_heading_attributes: bool,
    enable_tables: bool,
    enable_footnotes: bool,
    enable_strikethrough: bool,
    enable_tasklists: bool,
    enable_smart_punctuation: bool,
}

fuzz_target!(|data: &[u8]| {
    let mut opt = Options::empty();

    let mut u = Unstructured::new(data);
    if let Ok(opts) = Opts::arbitrary(&mut u) {
        if opts.enable_heading_attributes {
            opt.insert(pulldown_cmark_133::Options::ENABLE_HEADING_ATTRIBUTES);
        }
        if opts.enable_tables {
            opt.insert(pulldown_cmark_133::Options::ENABLE_TABLES);
        }
        if opts.enable_footnotes {
            opt.insert(pulldown_cmark_133::Options::ENABLE_FOOTNOTES);
        }
        if opts.enable_strikethrough {
            opt.insert(pulldown_cmark_133::Options::ENABLE_STRIKETHROUGH);
        }
        if opts.enable_tasklists {
            opt.insert(pulldown_cmark_133::Options::ENABLE_TASKLISTS);
        }
        if opts.enable_smart_punctuation {
            opt.insert(pulldown_cmark_133::Options::ENABLE_SMART_PUNCTUATION);
        }
    }
    let data_str = std::str::from_utf8(data).unwrap_or("");
    for _ in pulldown_cmark_133::Parser::new_ext(data_str, opt) {
    }
});
