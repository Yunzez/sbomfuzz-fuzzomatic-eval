use std::io::{ self, Cursor, BufReader, Read };

// *** main start
pub fn main() {
    println!("crate batch 3 starting");

    let input1 = b"\x04\x04\x04\x04:\x1az*\xfc\x06\x01\x90\x01\x06\x01";
    run_1(input1);
    
    let pdf_data = b"%PDF-1.5\n\
    000000028100 000 n \n\
    0000000338 00000 n \n\
    %%EOF";
    run_2(pdf_data);
    
    let lz4_data = b"\x04\x22\x4D\x18\x64\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    run_3(lz4_data);
    
    let compress_data = b"Some data to compress and decompress using lz4_flex";
    run_4(compress_data);
    
    let minidump_data = match base64::decode("U29tZSB2YWxpZCBkYXRhIHN0cmluZw==") {
        Ok(data) => data,
        Err(_) => vec![],
    };
    run_5(&minidump_data);

    run_8();
    
    let mp4_data = [0, 1, 102, 116, 121, 112, 0, 132, 255, 255, 255, 255, 0, 132];
    run_9(&mp4_data);
    
    let wgsl_data = [0, 0, 0, 1, 102, 116, 121, 112, 0, 132, 255, 255, 255, 255, 0, 132];
    run_10(&wgsl_data);
    
    let nom_data = [0; 1024];
    run_12(&nom_data);
    
    let npy_data: Vec<u8> = vec![
        147, 78, 85, 77, 80, 89, 1, 0, 118, 0, 123, 39, 100, 101, 115, 99,
        114, 39, 58, 32, 91, 40, 39, 60, 105, 52, 39, 44, 32, 52, 41, 93, 125
    ];
    run_13(&npy_data);
    
    let ntfs_data = [
        235, 82, 144, 78, 84, 70, 83, 32, 32, 32, 32, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 248, 0, 0, 0,
    ];
    run_14(&ntfs_data);
    
    run_15();
    println!("crate batch 3 ending");
}
// *** main end

pub fn run_1(input: &[u8]) {
    let mut decoder = libflate::deflate::Decoder::new(input);
    let result = io::copy(&mut decoder, &mut io::sink());
}

pub fn run_2(d: &[u8]) {
    let _ = lopdf::Document::load_mem(d);
}

use lz_fear::framed::{ CompressionSettings, LZ4FrameReader };
pub fn run_3(data: &[u8]) {
    let input = Cursor::new(data);
    let mut output = Vec::new();

    let lz4_reader = LZ4FrameReader::new(input);
    if let Ok(reader) = lz4_reader {
        let _ = reader.into_read().read_to_end(&mut output);
    }
}

use lz4_flex::block::decompress::decompress_size_prepended;
use lz4_flex::block::compress::compress_prepend_size;

pub fn run_4(data: &[u8]) {
    let compressed = compress_prepend_size(data);
    let decompressed = decompress_size_prepended(&compressed).unwrap();
}

use rust_minidump_86::{
    Minidump,
    MinidumpAssertion,
    MinidumpBreakpadInfo,
    MinidumpCrashpadInfo,
    MinidumpException,
    MinidumpLinuxCpuInfo,
    MinidumpLinuxEnviron,
    MinidumpLinuxLsbRelease,
    MinidumpLinuxMaps,
    MinidumpLinuxProcStatus,
    MinidumpMacCrashInfo,
    MinidumpMemoryInfoList,
    MinidumpMemoryList,
    MinidumpMiscInfo,
    MinidumpModuleList,
    MinidumpSystemInfo,
    MinidumpThreadList,
    MinidumpThreadNames,
    MinidumpUnloadedModuleList,
};

use minidump_processor_86::{ ProcessorOptions, process_minidump_with_options };

use std::error::Error;
use std::io::sink;

pub fn run_5(data: &[u8]) {
    rust_minidump_85::Minidump::read(data.to_vec());

    if let Ok(dump) = rust_minidump_86::Minidump::read(data.to_vec()) {
        let _ = dump.get_stream::<MinidumpAssertion>();
        let _ = dump.get_stream::<MinidumpBreakpadInfo>();
        let _ = dump.get_stream::<MinidumpCrashpadInfo>();
        let _ = dump.get_stream::<MinidumpException>();
        let _ = dump.get_stream::<MinidumpLinuxCpuInfo>();
        let _ = dump.get_stream::<MinidumpLinuxEnviron>();
        let _ = dump.get_stream::<MinidumpLinuxLsbRelease>();
        let _ = dump.get_stream::<MinidumpLinuxMaps>();
        let _ = dump.get_stream::<MinidumpLinuxProcStatus>();
        let _ = dump.get_stream::<MinidumpMacCrashInfo>();
        let _ = dump.get_stream::<MinidumpMemoryInfoList>();
        let _ = dump.get_stream::<MinidumpMemoryList>();
        let _ = dump.get_stream::<MinidumpMiscInfo>();
        let _ = dump.get_stream::<MinidumpModuleList>();
        let _ = dump.get_stream::<MinidumpSystemInfo>();
        let _ = dump.get_stream::<MinidumpThreadNames>();
        let _ = dump.get_stream::<MinidumpThreadList>();
        let _ = dump.get_stream::<MinidumpUnloadedModuleList>();
    }

    match rust_minidump_86::Minidump::read(data) {
        Ok(f) => {
            let e = f.get_stream::<MinidumpLinuxMaps>().unwrap_err();
            assert_eq!(e.to_string(), "Data error");
        }
        Err(e) => {
            println!("Expected to parse the header, got {:?}", e);
        }
    }
}
fn run_6() {
    // ? line 89
    // FIXME: this does not work, check the reference and made it work later

    // https://github.com/rust-minidump/rust-minidump/blob/main/minidump-processor/fuzz/fuzz_targets/process.rs
    // return;
    // let new_data = base64::decode("U29tZSBuZXcgdmFsaWQgZGF0YSBzdHJpbmc=").unwrap();

    // if let Ok(dump) = Minidump::read(&new_data[..]) {
    //     let supplier = StaticSymbolSupplier {
    //         file: new_data.clone(), // Use the decoded base64 data
    //     };

    //     let provider = Symbolizer::new(supplier);

    //     // Enable all unstable processing options
    //     let options = ProcessorOptions::unstable_all();

    //     // Process the minidump
    //     let val: Result<_, _> = fuzzing_block_on(
    //         process_minidump_with_options(&dump, &provider, options)
    //     );

    //     if let Ok(v) = val {
    //         let _: Result<(), _> = v.print_json(&mut sink(), true);
    //     }
    // }

    // ? line 90
    // let memory1 = rust_minidump_86::Minidump::Memory::with_section(
    //     rust_minidump_86::Minidump::Section::with_endian(Endian::Little).append_repeated(0, 2),
    //     u64::MAX,
    // );
    // let dump = rust_minidump_86::Minidump::SynthMinidump::with_endian(Endian::Little).add_memory(memory1);
    // let dump = rust_minidump_86::Minidump::read_synth_dump(dump).unwrap();
    // let memory_list = dump.get_stream::<MinidumpMemoryList<'_>>().unwrap();
    // let regions = memory_list.iter().collect::<Vec<_>>();
}
fn run_7() {
    // ? line 93, untestable since we have no access to minidump_processor_fuzz
    // let val: Result<_, _> = minidump_processor_fuzz::fuzzing_block_on(
    //     minidump_processor::process_minidump(&dump, &provider),
    // );
}

use std::panic;
use miniz_oxide::inflate::{ decompress_to_vec, core::DecompressorOxide };
use miniz_oxide::deflate::compress_to_vec;

pub fn run_8() {
    // Doesn't work - returns early
    return;
}

pub fn run_9(data: &[u8]) {
    let mut cursor = std::io::Cursor::new(data);
    let tag = mp4ameta::Tag::read_from(&mut cursor);
}

use naga::front::wgsl::Parser;
pub fn run_10(data: &[u8]) {
    let data_str = std::str::from_utf8(data).unwrap_or("");
    let _result = Parser::new().parse(data_str);
}
use std::str;
// use ncurses_rs::ToCStr;
fn run_11() {
    // ? line 101, trait not accessible anymore
    // let null: Vec<u8> = vec![0];
    // let null_str = str::from_utf8(&null).unwrap();

    // null_str.to_c_str();
}

#[macro_use]
extern crate nom;
use nom::{ IResult, Needed };
pub fn run_12(data: &[u8]) {
    named!(parser01<&[u8], ()>,
    do_parse!(
        hdr: take!(1) >>
        data: take!(1023) >>
        ( () )
    )
);

    let _ = parser01(data);
}

extern crate npy;
#[macro_use]
extern crate npy_derive;
#[derive(NpyData, Debug)]
struct Array {
    a: i32,
}
pub fn run_13(data: &[u8]) {
    let _ = npy::from_bytes::<Array>(data);
}

pub fn run_14(data: &[u8]) {
    let mut data_2 = std::io::Cursor::new(data);
    let _ = ntfs::Ntfs::new(&mut data_2);

    let mut mutdata = [
        235, 82, 144, 78, 84, 70, 83, 32, 32, 0, 0, 0, 0, 0, 0, 128, 32, 128, 0, 255, 15, 0, 0, 0,
        0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 255, 7, 0, 0, 0, 0, 0, 0, 149, 0, 0, 0, 8, 0, 0, 0, 120,
        183, 16, 124, 224, 39, 74, 127, 0, 0, 0, 0, 14, 31, 190, 113, 124, 172, 34, 192, 116, 11, 86,
        180, 14, 187, 7, 0, 205, 16, 94, 235, 240, 50, 228, 205, 22, 205, 25, 235, 254, 84, 104,
        105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 98, 111, 111, 116, 97, 98, 108, 101,
        32, 100, 105, 115, 107, 46, 32, 80, 50, 101, 97, 115, 101, 32, 105, 110, 115, 101, 114, 116,
        32, 97, 32, 98, 111, 111, 116, 97, 98, 108, 101, 32, 102, 108, 111, 112, 112, 121, 32, 97,
        110, 100, 13, 10, 112, 114, 101, 115, 115, 32, 97, 110, 121, 32, 107, 101, 121, 32, 116, 111,
        32, 116, 114, 121, 32, 97, 103, 97, 105, 110, 32, 97, 110, 121, 32, 107, 101, 121, 32, 116,
        111, 32, 116, 114, 121, 32, 97, 103, 97, 105, 110, 32, 46, 46, 46, 32, 13, 10, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 2,
        183, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 170,
    ];

    let mut cursor = std::io::Cursor::new(mutdata);
    if let Ok(mut fs) = ntfs::Ntfs::new(&mut cursor) {
        if let Err(e) = fs.read_upcase_table(&mut cursor) {
            eprintln!("Failed to read upcase table: {}", e);
        }
    } else {
        eprintln!("Failed to create NTFS filesystem");
    }
}


pub fn run_15() {
    // Disabled - commented out code
}
