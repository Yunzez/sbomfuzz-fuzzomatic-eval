#![allow(clippy::needless_return)]

#[macro_use]
extern crate nom;
extern crate npy;
#[macro_use]
extern crate npy_derive;

use base64;
use libflate::deflate::Decoder as DeflateDecoder;
use lopdf::Document;
use lz4_flex::block::compress::compress_prepend_size;
use lz4_flex::block::decompress::decompress_size_prepended;
use lz_fear::framed::LZ4FrameReader;
use mp4ameta::Tag;
use naga::front::wgsl::Parser;
use npy::NpyData;
use ntfs::Ntfs;
use rust_minidump_85::Minidump as Minidump85;
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
use std::error::Error;
use std::io::{self, Cursor, Read};

#[derive(NpyData, Debug)]
struct Array {
    a: i32,
}

pub struct BenchmarkData {
    pub run1_input: Vec<u8>,
    pub run2_pdf: Vec<u8>,
    pub run3_data: Vec<u8>,
    pub run4_text: Vec<u8>,
    pub run5_panic_b64: String,
    pub run5_data_b64: String,
    pub run9_data: Vec<u8>,
    pub run10_data: Vec<u8>,
    pub run12_data: Vec<u8>,
    pub run13_data: Vec<u8>,
    pub run14_data: Vec<u8>,
    pub run14_mutdata: Vec<u8>,
}

impl Default for BenchmarkData {
    fn default() -> Self {
        Self {
            run1_input: b"\x04\x04\x04\x04:\x1az*\xfc\x06\x01\x90\x01\x06\x01".to_vec(),
            run2_pdf: b"%PDF-1.5\n\
    000000028100 000 n \n\
    0000000338 00000 n \n\
    %%EOF".to_vec(),
            run3_data: b"\x04\x22\x4D\x18\x64\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec(),
            run4_text: b"Some data to compress and decompress using lz4_flex".to_vec(),
            run5_panic_b64: "TURNUJOnAAAA/2ZmZFlmZmZmZkAKCmZwCrv///8K/wo=".to_owned(),
            run5_data_b64: "U29tZSB2YWxpZCBkYXRhIHN0cmluZw==".to_owned(),
            run9_data: include_bytes!("run9_data.bin").to_vec(),
            run10_data: include_bytes!("run10_data.bin").to_vec(),
            run12_data: vec![0u8; 1024],
            run13_data: include_bytes!("run13_data.bin").to_vec(),
            run14_data: include_bytes!("run14_data.bin").to_vec(),
            run14_mutdata: include_bytes!("run14_mutdata.bin").to_vec(),
        }
    }
}

pub fn main() {
    println!("crate batch 3 benchmark starting");
    let data = BenchmarkData::default();
    benchmark(&data);
    println!("crate batch 3 benchmark ending");
}

pub fn benchmark(data: &BenchmarkData) {
    // --- run 1 ---------------------------------------------------------------
    {
        let mut decoder = DeflateDecoder::new(Cursor::new(&data.run1_input[..]));
        let _ = io::copy(&mut decoder, &mut io::sink());
    }

    // --- run 2 ---------------------------------------------------------------
    {
        let _ = Document::load_mem(&data.run2_pdf);
    }

    // --- run 3 ---------------------------------------------------------------
    {
        let input = Cursor::new(data.run3_data.clone());
        let mut output = Vec::new();

        if let Ok(reader) = LZ4FrameReader::new(input) {
            let _ = reader.into_read().read_to_end(&mut output);
        }
    }

    // --- run 4 ---------------------------------------------------------------
    {
        let compressed = compress_prepend_size(&data.run4_text);
        let _ = decompress_size_prepended(&compressed).unwrap();
    }

    // --- run 5 ---------------------------------------------------------------
    {
        let panic_data = base64::decode(data.run5_panic_b64.clone()).unwrap();
        let decoded = match base64::decode(data.run5_data_b64.clone()) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Failed to decode base64: {}", e);
                return;
            }
        };
        Minidump85::read(decoded.clone());

        if let Ok(dump) = Minidump::read(decoded.clone()) {
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

        match Minidump::read(&decoded[..]) {
            Ok(f) => {
                let e = f.get_stream::<MinidumpLinuxMaps>().unwrap_err();
                assert_eq!(e.to_string(), "Data error");
            }
            Err(e) => {
                println!("Expected to parse the header, got {:?}", e);
            }
        }

        let _ = panic_data;
    }

    // --- run 8 ---------------------------------------------------------------
    {
        // Original implementation returns immediately; nothing to execute here.
    }

    // --- run 9 ---------------------------------------------------------------
    {
        let mut cursor = Cursor::new(data.run9_data.clone());
        let _ = Tag::read_from(&mut cursor);
    }

    // --- run 10 --------------------------------------------------------------
    {
        let data_str = std::str::from_utf8(&data.run10_data).unwrap_or("");
        let _ = Parser::new().parse(data_str);
    }

    // --- run 12 --------------------------------------------------------------
    {
        named!(parser01<&[u8], ()>,
            do_parse!(
                hdr: take!(1) >>
                data: take!(1023) >>
                ( () )
            )
        );

        let _ = parser01(&data.run12_data);
    }

    // --- run 13 --------------------------------------------------------------
    {
        let _ = npy::from_bytes::<Array>(&data.run13_data);
    }

    // --- run 14 --------------------------------------------------------------
    {
        let mut cursor = Cursor::new(data.run14_data.clone());
        let _ = Ntfs::new(&mut cursor);

        let mut cursor = Cursor::new(data.run14_mutdata.clone());
        if let Ok(mut fs) = Ntfs::new(&mut cursor) {
            if let Err(e) = fs.read_upcase_table(&mut cursor) {
                eprintln!("Failed to read upcase table: {}", e);
            }
        } else {
            eprintln!("Failed to create NTFS filesystem");
        }
    }

    // --- run 15 --------------------------------------------------------------
    {
        // Intentionally left blank as in source.
    }
}
