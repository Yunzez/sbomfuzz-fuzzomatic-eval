use std::io::{ self, Cursor, BufReader, Read };

// *** main start
pub fn main() {
    println!("crate batch 4 starting");
    
    run_1();
    
    let pcap_data = b"\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19";
    run_2(pcap_data);
    
    let phone_data = "2067736026";
    run_3(phone_data);
    
    let pdf_data = b"%PDF-startxref>";
    // ! comment out this because fuzzomatic stuck on this 
    // run_4(pdf_data); 
    let mut lexer = pdf_115::parser::Lexer::new(b"0 1\n0000000000 65535 f\n");
    run_5(&mut lexer);
    
    let pgp_data = [5, 2, 2, 11, 0, 2, 0, 0];
    run_6(&pgp_data);
    
    let plist_data = b"bplist00\x10\x01";
    run_7(plist_data);
    
    let png_data: &[u8] = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\nIDATx\xdac\xf8\x0f\x00\x01\x01\x01\x00\x18\xdd\x03\x1a\x00\x00\x00\x00IEND\xaeB`\x82";
    run_8(png_data);
    
    let table_data: Vec<Vec<(String, u8)>> = vec![
        vec![("Cell1".to_string(), 0), ("Cell2".to_string(), 1), ("Cell3".to_string(), 2)],
        vec![("Cell4".to_string(), 0), ("Cell5".to_string(), 1), ("Cell6".to_string(), 2)],
    ];
    run_9(table_data);
    
    let md_data = "01010101";
    run_10(md_data);
    
    let xml_data: &[u8] = b"\x00";
    run_11(xml_data);
    
    println!("crate batch 4 ending");
}
// *** main end

use std::ffi::{ CString, NulError };
use std::str;

pub fn run_1() {
    // Disabled - pancurses not working
}

pub fn run_2(data: &[u8]) {
    let _ = pcapng::block::parse_block(data);
}

pub fn run_3(data: &str) {
    phonenumber::parse(None, data);
}

// pub fn run_4(data: &[u8]) {
//     pdf_112::file::File::from_data(data);
// }

use glob::glob;
use pdf_115::file::File;
pub fn run_5(lexer: &mut pdf_115::parser::Lexer) {
    
    let resolve = pdf_115::object::NoResolve;

    match pdf_115::parser::parse_xref_stream_and_trailer(lexer, &resolve) {
        Ok((xref_sections, dictionary)) => {
            println!("Parsed xref sections: {:?}", xref_sections);
            println!("Parsed dictionary: {:?}", dictionary);
        }
        Err(e) => println!("Failed to parse xref stream and trailer: {:?}", e),
    }

    for entry in glob("invalid/*.pdf").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let path = path.to_str().unwrap();
                println!("\n\n == Now testing `{}` ==\n", path);

                match File::<Vec<u8>>::open(path) {
                    Ok(file) => {
                        for i in 0..file.num_pages() {
                            let _ = file.get_page(i);
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
            Err(e) => panic!("error when reading glob patterns: {:?}", e),
        }
    }
}

pub fn run_6(data: &[u8]) {
    let _ = pgp::Signature::from_slice(pgp::types::Version::New, data);
}

pub fn run_7(data: &[u8]) {
    let cursor = Cursor::new(data);
    plist::Value::from_reader(cursor);
}

extern crate png;

fn decode_png(data: &[u8]) -> io::Result<Vec<u8>> {
    let limits = png::Limits { bytes: 1 << 16 };
    let decoder = png::Decoder::new_with_limits(data, limits);
    let (info, mut reader) = decoder.read_info()?;

    if info.buffer_size() > 5_000_000 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "memory limit exceeded"));
    }

    let mut data = vec![0u8; info.buffer_size()];
    reader.next_frame(&mut data)?;

    Ok(data)
}

pub fn run_8(data: &[u8]) {
    decode_png(data);
}

pub fn run_9(table_data: Vec<Vec<(String, u8)>>) {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    for row in table_data {
        let mut table_row = Vec::new();
        for (s, n) in row {
            table_row.push(format!("{}: {}", s, n));
        }
        table.add_row(Row::new(table_row.iter().map(|s| Cell::new(s)).collect()));
    }

    let _ = table.to_string();
}

pub fn run_10(data: &str) {
    pulldown_cmark_128::Parser::new(data);
    
    let mut output = String::new();
    let parser = pulldown_cmark_131::Parser::new(data);
    pulldown_cmark_131::html::push_html(&mut output, parser);
    
    let mut opts = pulldown_cmark_133::Options::empty();
    opts.insert(pulldown_cmark_133::Options::ENABLE_HEADING_ATTRIBUTES);
    for _ in pulldown_cmark_133::Parser::new_ext(data, opts) {
    }
}

pub fn run_11(data: &[u8]) {
    let cursor = Cursor::new(data);
    let mut reader = Reader::from_reader(cursor);
    reader.trim_text(true);
    let mut buf = vec![];
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Eof) | Err(..) => break,
            _ => buf.clear(),
        }
    }
}

use prettytable::{Cell, Row, Table};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn align_from_u8(x: u8) -> prettytable::format::Alignment {
    match x {
        0 => prettytable::format::Alignment::LEFT,
        1 => prettytable::format::Alignment::CENTER,
        _ => prettytable::format::Alignment::RIGHT,
    }
}

fn run_table(data: Vec<Vec<(String, u8)>>) {
    let mut pt = prettytable::Table::new();
    for row in data {
        let cells = row
            .into_iter()
            .map(|x| Cell::new_align(&x.0, align_from_u8(x.1)))
            .collect();
        pt.add_row(Row::new(cells));
    }

    let _ = pt.print(&mut std::io::sink());
}
