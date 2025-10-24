#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use juniper::parser::Lexer;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    input: &'a str,
}

fuzz_target!(|data: FuzzInput| {
    let mut lexer = Lexer::new(data.input);

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
            Some(Err(_e)) => {
                return;
            }
            None => {
                break;
            }
        }
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: juniper 
//  - Crate Link: None 
//  - Crate Version: 0.14.2 
//  - From Crate: crate_batch_2 
//  - From Crate Link: unknown_website 
//  - Module Path: juniper::parser::lexer::(Struct)Lexer 
//  - Use Statement: use  juniper::parser::Lexer 
//  - Function Name: new 
//  - Function Usage: fn run_update() {
//         // !add juniper 
//         let s = "query { human(id: \"1000\") { name } }";
//         let mut tokens = Vec::new();
//         let mut lexer = Lexer::new(s);
//     
//     
//         loop {
//             match lexer.next() {
//                 Some(Ok(t)) => {
//                     let at_eof = t.item == Token::EndOfFile;
//                     tokens.push(t);
//                     if at_eof {
//                         break;
//                     }
//                 }
//                 Some(Err(e)) => panic!("Error in input stream: {:#?} for {:#?}", e, s),
//                 None => panic!("EOF before EndOfFile token in {:#?}", s),
//             }
//         }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a str) -> Lexer<'a>,
//   type_fields: [juniper::Lexer] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a str) -> Lexer<'a>,
//   type_fields: [juniper::Lexer] 
// }
// Struct construction metadata: {
//   type_name: juniper::Lexer,
//   type_fields: [core::Peekable, &'a str, usize, juniper::SourcePosition, bool] 
// }
// Struct construction metadata: {
//   type_name: core::Peekable,
//   type_fields: [core::CharIndices, core::Option] 
// }
// Struct construction metadata: {
//   type_name: core::CharIndices,
//   type_fields: [usize, core::Chars] 
// }
// Struct construction metadata: {
//   type_name: core::Chars,
//   type_fields: [core::Iter] 
// }
// Struct construction metadata: {
//   type_name: core::Iter,
//   type_fields: [core::NonNull, *const u8, core::PhantomData] 
// }
// Struct construction metadata: {
//   type_name: core::NonNull,
//   type_fields: [*const u8] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: juniper::SourcePosition,
//   type_fields: [usize, usize, usize] 
// }

