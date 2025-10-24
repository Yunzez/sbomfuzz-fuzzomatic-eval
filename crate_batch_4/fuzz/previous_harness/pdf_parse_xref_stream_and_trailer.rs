#![no_main]
extern crate libfuzzer_sys;
extern crate pdf_115;
use libfuzzer_sys::fuzz_target;
use pdf_115::parser::Lexer;
use pdf_115::parser::parse_xref_stream_and_trailer;
use pdf_115::object::NoResolve;

fuzz_target!(|data: &[u8]| {
    let resolve = NoResolve;
    let mut lexer = Lexer::new(data);

    match parse_xref_stream_and_trailer(&mut lexer, &resolve)
    {
        Ok((xref_sections, dictionary)) => {
            println!("Parsed xref sections: {:?}", xref_sections);
            println!("Parsed dictionary: {:?}", dictionary);
        }
        Err(e) => println!("Failed to parse xref stream and trailer: {:?}", e),
    }
});