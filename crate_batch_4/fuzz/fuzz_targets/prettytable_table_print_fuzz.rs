#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use prettytable::{Table, Row, Cell};
use prettytable::format::Alignment::{self, *};
#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<Vec<(String, u8)>>
}

fuzz_target!(|input: FuzzInput| {
    let mut table = Table::new();
    for row_data in input.data {
        let cells: Vec<Cell> = row_data.into_iter().map(|(text, alignment)| {
            let align = match alignment % 3 {
                0 => Alignment::LEFT,
                1 => Alignment::CENTER,
                _ => Alignment::RIGHT,
            };
            Cell::new_align(&text, align)
        }).collect();
        
        table.add_row(Row::new(cells));
    }

    let _ = table.print(&mut std::io::sink()).unwrap_or_default();
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
//  - Module Path: prettytable::cell::(Struct)Cell 
//  - Use Statement: use prettytable::Cell 
//  - Function Name: new_align 
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
//   type_name: fn(&str, Alignment) -> Cell,
//   type_fields: [prettytable::Alignment, prettytable::Cell] 
// }
// Struct construction metadata: {
//   type_name: fn(&str, Alignment) -> Cell,
//   type_fields: [prettytable::Alignment, prettytable::Cell] 
// }
// Struct construction metadata: {
//   type_name: prettytable::Alignment,
//   type_fields: [LEFT, CENTER, RIGHT] 
// }
// Struct construction metadata: {
//   type_name: prettytable::Cell,
//   type_fields: [alloc::Vec, usize, prettytable::Alignment, alloc::Vec, usize] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [pdf::XRef, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: pdf::XRef,
//   type_fields: [Free, Raw, Stream, Promised, Invalid] 
// }

