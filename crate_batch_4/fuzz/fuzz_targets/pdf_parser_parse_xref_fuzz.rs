#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use pdf_115::parser::parse_xref_stream_and_trailer;
use pdf_115::parser::Lexer;
use pdf_115::object::NoResolve;

// Arbitrary implementation for parser fuzzing
#[derive(Arbitrary, Debug)]
struct LexerInput<'a> {
    input: &'a [u8],
}

fuzz_target!(|data: LexerInput| {
    let mut lexer = Lexer::new(data.input);
    let resolve = NoResolve;

    // Fuzz the parse_xref_stream_and_trailer function
    let _ = parse_xref_stream_and_trailer(&mut lexer, &resolve);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: pdf 
//  - Crate Link: None 
//  - Crate Version: 0.7.2 
//  - From Crate: crate_batch_4 
//  - From Crate Link: unknown_website 
//  - Module Path: pdf::file::(Struct)File 
//  - Use Statement: use pdf_115::file::File; 
//  - Function Name: open 
//  - Function Usage: fn run_5() {
//     //? pdf with hash 99e70cd
//     println!("running pdf_115");
//     // info: line 115 & 117
//     let mut lexer = pdf_115::parser::Lexer::new(b"0 1\n0000000000 65535 f\n");
//     let resolve = pdf_115::object::NoResolve;
// 
//     match pdf_115::parser::parse_xref_stream_and_trailer(&mut lexer, &resolve) {
//         Ok((xref_sections, dictionary)) => {
//             println!("Parsed xref sections: {:?}", xref_sections);
//             println!("Parsed dictionary: {:?}", dictionary);
//         }
//         Err(e) => println!("Failed to parse xref stream and trailer: {:?}", e),
//     }
// 
//     // info: line 116
// 
//     for entry in glob("invalid/*.pdf").expect("Failed to read glob pattern") {
//         match entry {
//             Ok(path) => {
//                 let path = path.to_str().unwrap();
//                 println!("\n\n == Now testing `{}` ==\n", path);
// 
//                 match File::<Vec<u8>>::open(path) {
//                     Ok(file) => {
//                         for i in 0..file.num_pages() {
//                             let _ = file.get_page(i);
//                         }
//                     }
//                     Err(_) => {
//                         continue;
//                     }
//                 }
//             }
//             Err(e) => panic!("error when reading glob patterns: {:?}", e),
//         }
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(impl AsRef<Path>) -> Result<File<Vec<u8, Global>>, PdfError>,
//   type_fields: [std::Path, alloc::Global, pdf::PdfError] 
// }
// Struct construction metadata: {
//   type_name: fn(impl AsRef<Path>) -> Result<File<Vec<u8, Global>>, PdfError>,
//   type_fields: [std::Path, alloc::Global, pdf::PdfError] 
// }
// Struct construction metadata: {
//   type_name: std::Path,
//   type_fields: [std::OsStr] 
// }
// Struct construction metadata: {
//   type_name: std::OsStr,
//   type_fields: [std::Slice] 
// }
// Struct construction metadata: {
//   type_name: std::Slice,
//   type_fields: [[u8]] 
// }
// Struct construction metadata: {
//   type_name: pdf::PdfError,
//   type_fields: [EOF, NoOpArg, Parse, Encoding, Bounds, UnexpectedLexeme, UnknownType, UnknownVariant, NotFound, Reference, XRefStreamType, ContentReadPastBoundary, HexDecode, Ascii85TailError, IncorrectPredictorType, FromPrimitive, MissingEntry, KeyValueMismatch, WrongDictionaryType, FreeObject, NullRef, UnexpectedPrimitive, ObjStmOutOfBounds, PageOutOfBounds, PageNotFound, UnspecifiedXRefEntry, InvalidPassword, DecryptionFailure, Jpeg, Io, Other, NoneError, Try, TryContext, PostScriptParse, PostScriptExec] 
// }

