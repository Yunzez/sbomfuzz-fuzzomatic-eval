#![no_main]
use libfuzzer_sys::fuzz_target;
use quick_xml::reader::Reader;
use std::string::String;

fuzz_target!(|data: String| {
    let mut reader = Reader::from_str(&data);
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event(&mut buf) {
            Ok(quick_xml::events::Event::Eof) | Err(_) => {
                break;
            }
            _ => buf.clear(),
        }
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: quick_xml 
//  - Crate Link: None 
//  - Crate Version: 0.6.0 
//  - From Crate: crate_batch_4 
//  - From Crate Link: unknown_website 
//  - Module Path: quick_xml::reader::(Struct)Reader 
//  - Use Statement: use quick_xml::reader::Reader; 
//  - Function Name: from_str 
//  - Function Usage: fn run_11() {
//     // ! crashing input
//     // let data : &[u8] = b"\xe9\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\n(\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00<>\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00<<\x00\x00\x00";
// 
//     // ? line 134， 135, 136
//     let data: &[u8] = b"\x00";
//     let cursor = Cursor::new(data);
//     let mut reader = Reader::from_reader(cursor);
//     reader.trim_text(true); // ! this is for line 136, it only takes one line
//     let mut buf = vec![];
//     loop {
//         match reader.read_event(&mut buf) {
//             Ok(quick_xml::events::Event::Eof) | Err(..) => {
//                 break;
//             }
//             _ => buf.clear(),
//         }
//     }
// 
//     // ? line 137
//     let doc = "<!D>";
//     let mut reader = Reader::from_str(doc);
//     let mut buf = Vec::new();
//     reader.read_event(&mut buf);
//     reader.read_event(&mut buf);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a str) -> Reader<&'a [u8]>,
//   type_fields: [quick_xml::Reader] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a str) -> Reader<&'a [u8]>,
//   type_fields: [quick_xml::Reader] 
// }
// Struct construction metadata: {
//   type_name: quick_xml::Reader,
//   type_fields: [&'a [u8], bool, usize, quick_xml::TagState, bool, bool, bool, bool, alloc::Vec, alloc::Vec, quick_xml::NamespaceBuffer, &'static Encoding] 
// }
// Struct construction metadata: {
//   type_name: quick_xml::TagState,
//   type_fields: [Opened, Closed, Empty] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [pdf::XRef, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: pdf::XRef,
//   type_fields: [Free, Raw, Stream, Promised, Invalid] 
// }
// Struct construction metadata: {
//   type_name: quick_xml::NamespaceBuffer,
//   type_fields: [alloc::Vec, alloc::Vec, i32, bool] 
// }

