#![no_main]
use libfuzzer_sys::fuzz_target;
use juniper::parser::Lexer;
use arbitrary::Arbitrary;

fuzz_target!(|data: &str| {
    let mut lexer = Lexer::new(data);
    let mut tokens = Vec::new();

    loop {
        match lexer.next() {
            Some(Ok(t)) => {
                let at_eof = t.item == juniper::parser::Token::EndOfFile;
                tokens.push(t);
                if at_eof {
                    break;
                }
            }
            Some(Err(_e)) => return, // simply return on error to prevent panic
            None => return, // EOF before EndOfFile token, exit the loop
        }
    }
});