#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use prettytable::{Table, Row, Cell, Alignment};
use prettytable::format::Alignment::{self, *};
#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<(String, u8)>,
}

fuzz_target!(|input: FuzzInput| {
    let mut table = Table::new();
    let cells: Vec<Cell> = input.data.iter()
        .map(|(string, alignment)| {
            let align = match alignment % 3 {
                0 => Alignment::LEFT,
                1 => Alignment::CENTER,
                _ => Alignment::RIGHT,
            };
            Cell::new_align(string, align)
        })
        .collect();
    let row = Row::new(cells);
    table.add_row(row);
    let _ = table.print(&mut std::io::sink());
});