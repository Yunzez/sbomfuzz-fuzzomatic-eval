#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use prettytable::{Row, Cell};
use prettytable::format::Alignment::{self, *};
#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<(String, u8)>
}

fuzz_target!(|input: FuzzInput| {
    let cells: Vec<Cell> = input.data.into_iter().map(|(text, alignment)| {
        let align = match alignment % 3 {
            0 => Alignment::LEFT,
            1 => Alignment::CENTER,
            _ => Alignment::RIGHT,
        };
        Cell::new_align(&text, align)
    }).collect();

    let _ = Row::new(cells);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: prettytable 
//  - Crate Link: unknown_website 
//  - Crate Version: 0.9.0 
//  - From Crate: crate_batch_4 
//  - From Crate Link: unknown_website 
//  - Module Path: prettytable::(Struct)Table 
//  - Use Statement: None 
//  - Function Name: add_row 
//  - Function Usage: fn run_table(data: Vec<Vec<(String, u8)>>) {
//     let mut pt = prettytable::Table::new();
//     for row in data {
//         let cells = row
//             .into_iter()
//             .map(|x| Cell::new_align(&x.0, align_from_u8(x.1)))
//             .collect();
//         pt.add_row(Row::new(cells));
//     }
// 
//     let _ = pt.print(&mut std::io::sink());
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut Table, Row) -> &mut Row,
//   type_fields: [prettytable::Table, prettytable::Row] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut Table, Row) -> &mut Row,
//   type_fields: [prettytable::Table, prettytable::Row] 
// }
// Struct construction metadata: {
//   type_name: prettytable::Table,
//   type_fields: [alloc::Box, alloc::Box, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: alloc::Box,
//   type_fields: [plist::ErrorImpl, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: plist::ErrorImpl,
//   type_fields: [plist::ErrorKind, core::Option] 
// }
// Struct construction metadata: {
//   type_name: plist::ErrorKind,
//   type_fields: [UnexpectedEof, UnexpectedEndOfEventStream, UnexpectedEventType, UnclosedXmlElement, UnpairedXmlClosingTag, UnexpectedXmlCharactersExpectedElement, UnexpectedXmlOpeningTag, UnknownXmlElement, InvalidXmlSyntax, InvalidXmlUtf8, InvalidDataString, InvalidDateString, InvalidIntegerString, InvalidRealString, UidNotSupportedInXmlPlist, ObjectTooLarge, InvalidMagic, InvalidTrailerObjectOffsetSize, InvalidTrailerObjectReferenceSize, InvalidObjectLength, ObjectReferenceTooLarge, ObjectOffsetTooLarge, RecursiveObject, NullObjectUnimplemented, FillObjectUnimplemented, IntegerOutOfRange, InfiniteOrNanDate, InvalidUtf8String, InvalidUtf16String, UnknownObjectType(u8), Io(io::Error), Serde(String)] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
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
//   type_name: prettytable::Row,
//   type_fields: [alloc::Vec] 
// }

