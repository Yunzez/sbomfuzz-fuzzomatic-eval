// *** main start
pub fn main() {
    println!("crate batch 5 starting");

    let rasn_data = [24, 19, 43, 53, 49, 54, 49, 53, 32, 32, 48, 53, 50, 52, 48, 57, 52, 48, 50, 48, 90];
    run_1(&rasn_data);
    
    run_4();
    
    let ron_data = "{}";
    run_3(ron_data);
    
    run_5();
    run_6();
    
    let yaml_data = b"invalid yaml data";
    run_7(yaml_data);
    
    let asn1_data = &[0x30, 0x0a, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02, 0x02, 0x01];
    run_8(asn1_data);
    
    let snap_data = &[0x82, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00, 0x00];
    run_9(snap_data);
    
    run_11();
    
    let sql_data = "(";
    run_12(sql_data);
    
    let ssh_key_data = "-----BEGIN OPENSSH PRIVATE KEY------END OPENSSH PRIVATE KEY-----ENSSH PRIVAPRIVATE KEY-----\x00\x00\x00\x01\x00";
    run_13(ssh_key_data);
    
    let ssh_packet_data = b"\x00\x00\x00\x00\x00\x00\x00\x00";
    run_14(ssh_packet_data);
    
    let swf_data = b"CWSCCCACCGCCC";
    run_15(swf_data);
    
    run_16();
    
    let symphonia_data = vec![0xff, 0xf1, 0xaf, 0xce, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xfb, 0xaf];
    run_17(symphonia_data);
    
    let syn_data = "6E--5458";
    run_18(syn_data);
    
    run_19();
    
    let sqlformat_data = "?62666666121266666612";
    run_20(sqlformat_data);
    
    println!("crate batch 5 ending");
}
// *** main end

pub fn run_1(data: &[u8]) {
    let value = match rasn::der::decode::<rasn::types::Open>(data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Decoding failed: {:?}", e);
            return;
        }
    };

    let encoded = rasn::der::encode(&value).unwrap();

    match rasn::der::decode::<rasn::types::Open>(&encoded) {
        Ok(_) => println!("Decoding succeeded!"),
        Err(e) => eprintln!("Decoding failed after encoding: {:?}", e),
    }
}

fn run_2() {}

pub fn run_3(data: &str) {
    println!("running line 152");
    let result: Result<ron::Value, _> = ron::from_str(data);
    match result {
        Ok(value) => println!("Parsed successfully: {:?}", value),
        Err(err) => eprintln!("Failed to parse: {}", err),
    }

    println!("running line 153");
    let v: ron::Value = ron::from_str(data).expect("Valid input should not fail");
    println!("{:?}", v);

    let ser = ron::to_string(&v).unwrap();
    println!("{:?}", ser);
}

pub fn run_4() {
    ini::Ini::read_from(&mut std::io::Cursor::new(String::from(""))).unwrap();
}

pub fn run_5() {
    if let Err(e) = std::panic::catch_unwind(|| {
        rustc_demangle::demangle("_ZN2222222222222222222222EE");
    }) {
        eprintln!("Caught panic in run 5: {:?}", e);
    }
}

pub fn run_6() {
    if let Err(e) = std::panic::catch_unwind(|| {
        semver::VersionReq::parse("8.*.7").unwrap();
    }) {
        eprintln!("Caught panic in run 6: {:?}", e);
    }
}

use serde_yaml::Number;
pub fn run_7(data: &[u8]) {
    match serde_yaml::from_slice::<serde_yaml::Value>(data) {
        Ok(value) => println!("Parsed YAML successfully: {:?}", value),
        Err(err) => eprintln!("Failed to parse YAML: {}", err),
    }

    let yaml_str = "50.";
    let deserialized: Number = serde_yaml::from_str(yaml_str).unwrap();

    println!("Deserialized from YAML (50.): {:?}", deserialized);

    let serialized_yaml = serde_yaml::to_string(&deserialized).unwrap();
    println!("Serialized to YAML: {}", serialized_yaml);

    let roundtrip: Number = serde_yaml::from_str(&serialized_yaml).unwrap();
    println!("Deserialized again: {:?}", roundtrip);

    assert_eq!(deserialized, roundtrip, "Roundtrip failed!");
}

pub fn run_8(data: &[u8]) {
    if let Err(e) = std::panic::catch_unwind(|| {
        let _ = simple_asn1::from_der(data);
    }) {
        eprintln!("Caught panic in run 8: {:?}", e);
    }
}

use snap::Decoder;
pub fn run_9(data: &[u8]) {
    let mut decoder = Decoder::new();

    match decoder.decompress_vec(data) {
        Ok(_) => println!("Decompression succeeded."),
        Err(e) => eprintln!("Decompression failed: {:?}", e),
    }
}

use core::cmp::{ min, Ordering };
use soroban_env_host::budget::{ Budget };
use soroban_env_host::{ Compare, Host };
use soroban_env_host::TryFromVal;
use soroban_env_common::xdr::ScVec;
use soroban_env_common::{ Symbol };
pub fn run_11() {
    println!("run 11");
    let v1 = ScVec::try_from((0, 1)).unwrap();
    let v2 = ScVec::try_from((0, 0, 2)).unwrap();
    let expected_cmp = Ordering::Greater;
    let budget = Budget::default();
    let actual_cmp = budget.compare(&v1, &v2).unwrap();

    let host = Host::default();
    let s = "#";
    let s = Symbol::try_from_val(&host, &s);
    match s {
        Ok(_) => println!("Unexpected success in symbol conversion"),
        Err(_) => println!("Symbol conversion failed as expected"),
    }
}

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn run_12(sql: &str) {
    println!("run 12");
    let dialect = GenericDialect {};

    if let Err(e) = std::panic::catch_unwind(|| {
        Parser::parse_sql(&dialect, sql);
    }) {
        eprintln!("Caught panic in run 12: {:?}", e);
    }
}

pub fn run_13(data: &str) {
    println!("run 13");

    if let Err(e) = std::panic::catch_unwind(|| {
        ssh_keys::openssh::parse_private_key(data);
    }) {
        eprintln!("Caught panic in run 13: {:?}", e);
    }
}

pub fn run_14(data: &[u8]) {
    println!("run 14");

    if let Err(e) = std::panic::catch_unwind(|| {
        let _ = ssh_parser::parse_ssh_packet(data);
    }) {
        eprintln!("Caught panic in run 14: {:?}", e);
    }
}

use swf_parser::streaming::movie;
pub fn run_15(bytes: &[u8]) {
    if let Err(e) = std::panic::catch_unwind(|| {
        let _ = movie::parse_movie(bytes);
    }) {
        eprintln!("Caught panic in run 15: {:?}", e);
    }
}

use symbolic::common::ByteView;
pub fn run_16() {
    println!("run 16");
    symbolic::demangle::demangle(
        "_ZUlzjjlZZL1zStUlSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjtUlSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjzL4t7IjIjjzjjzSt7j_Z3kjIIjfjzStfjzSt7j_ZA3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjzL4t7IjIjjzjjzSt7j_Z3kjIIjfjzStfjzSt7j_ZA3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjzL4t7IjIjL1vfIIEEEjzjjSI"
    );

    let data =
        b"MDMP\x93\xa7\x00\x00\r\x00\x00\x00 \xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    let bv = ByteView::from_slice(data);

    let data =
        b"MDMP\x93\xa7\x00\x00\r\x00\x00\x00 \xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    let _ = symbolic::unreal::Unreal4Crash::parse_with_limit(data, 1024 * 1024);
}

use symphonia::core::codecs::{ CodecParameters, DecoderOptions };
use symphonia::core::errors;
use symphonia::core::formats::{ FormatOptions, FormatReader };
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::core::meta::MetadataOptions;
use symphonia::default::get_probe;
use std::io::Cursor;

fn test_decode(data: Vec<u8>) -> symphonia::core::errors::Result<()> {
    let data = Cursor::new(data);
    let mss = MediaSourceStream::new(Box::new(data), Default::default());

    let probed = get_probe().format(
        &Hint::new(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default()
    )?;
    let mut reader = probed.format;

    let track = reader.default_track().ok_or(errors::Error::DecodeError("No default track found"))?;
    let params = track.codec_params.clone();

    println!("creating decoder");
    let mut decoder = match
        std::panic::catch_unwind(|| {
            symphonia::default::get_codecs().make(&params, &DecoderOptions::default())
        })
    {
        Ok(Ok(decoder)) => decoder,
        Ok(Err(e)) => {
            return Err(e);
        }
        Err(_) => {
            return Err(errors::Error::DecodeError("Panic occurred while creating decoder"));
        }
    };
    println!("entering while decode loop");
    while let Ok(packet) = reader.next_packet() {
        let _ = decoder.decode(&packet);
    }

    Ok(())
}

pub fn run_17(file: Vec<u8>) {
    println!("run 17");
    let err = test_decode(file).unwrap_err();
}

use syn_188::Expr;

pub fn run_18(s: &str) {
    println!("run 18");
    if let Err(e) = std::panic::catch_unwind(|| {
        let _ = syn_188::parse_str::<Expr>(s);
    }) {
        eprintln!("Caught panic in run 18: {:?}", e);
    }
}

pub fn run_19() {
    let context = tera_190::Context::new();
    if let Err(e) = std::panic::catch_unwind(|| {
        let _ = tera_190::Tera::one_off("{{1/0}}", &context, true);
    }) {
        eprintln!("Caught panic in run 19: {:?}", e);
    }
}

use sqlformat::*;
use sqlformat::FormatOptions as SqlFormatOptions;
pub fn run_20(data: &str) {
    if let Err(e) = std::panic::catch_unwind(|| {
        let _ = format(
            data,
            &QueryParams::None,
            SqlFormatOptions::default()
        );
    }) {
        eprintln!("Caught panic in run 20: {:?}", e);
    }
}

use raven_uxn::{ Backend, EmptyDevice, Uxn, UxnRam };

pub fn run_update() {
    let mut ram_v = UxnRam::new();
    let mut vm_v = Uxn::new(&mut ram_v, Backend::Interpreter);


    // Don't load any programs that require auxiliary memory
    let data: &[u8] = &[0x00, 0x01, 0x02, 0x03]; // Random data
    if !vm_v.reset(data).is_empty() {
        return;
    }
}

use unified_diff::diff;
pub fn run_update_2() {
    let test_cases = vec![
        (vec![b'a', b'b', b'c'], vec![b'x', b'y', b'z'], 3),
        (vec![b'1', b'2', b'3'], vec![b'4', b'5', b'6'], 2),
        (vec![b'!', b'@', b'#'], vec![b'$', b'%', b'^'], 1)
    ];

    for (from, to, context) in test_cases {
        if let Ok(s) = String::from_utf8(from.clone()) {
            if !s.is_ascii() {
                continue;
            }
            if s.find(|x| x < ' ' && x != '\n').is_some() {
                continue;
            }
        } else {
            continue;
        }
        if let Ok(s) = String::from_utf8(to.clone()) {
            if !s.is_ascii() {
                continue;
            }
            if s.find(|x| x < ' ' && x != '\n').is_some() {
                continue;
            }
        } else {
            continue;
        }
        let diff_result = diff(&from, "a/fuzz.file", &to, "target/fuzz.file", context as usize);
        println!("Diff result: {:?}", diff_result);
    }
}
