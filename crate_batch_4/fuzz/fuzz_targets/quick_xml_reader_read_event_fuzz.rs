#![no_main]
use libfuzzer_sys::fuzz_target;
use quick_xml::reader::Reader;
use std::io::Cursor;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let cursor = Cursor::new(&input.data);
    let mut reader = Reader::from_reader(cursor);
    reader.trim_text(true);

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
//  - Use Statement: None 
//  - Function Name: read_event 
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
//   type_name: fn(&'a mut Reader<B>, &'b mut Vec<u8, Global>) -> Result<Event<'b>, Error>,
//   type_fields: [B, alloc::Global, quick_xml::Event, quick_xml::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a mut Reader<B>, &'b mut Vec<u8, Global>) -> Result<Event<'b>, Error>,
//   type_fields: [B, alloc::Global, quick_xml::Event, quick_xml::Error] 
// }
// Struct construction metadata: {
//   type_name: quick_xml::Event,
//   type_fields: [Start(BytesStart<'a>), End(BytesEnd<'a>), Empty(BytesStart<'a>), Text(BytesText<'a>), Comment(BytesText<'a>), CData(BytesText<'a>), Decl(BytesDecl<'a>), PI(BytesText<'a>), DocType(BytesText<'a>), Eof] 
// }
// Struct construction metadata: {
//   type_name: quick_xml::Error,
//   type_fields: [{unknown}, error_chain::State] 
// }
// Struct construction metadata: {
//   type_name: error_chain::State,
//   type_fields: [core::Option, core::Option] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }

