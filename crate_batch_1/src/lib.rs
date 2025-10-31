//! This module contains various functions for demonstrating and testing different Rust features.
//! It includes examples of parsing, encoding, and other operations.

use alloy_json_abi::JsonAbi;
use std::io::Cursor;
use std::io::BufReader;
use std::time::Duration;

// *** main start
/// The entry point of the crate. This function demonstrates the usage of various `run_*` functions.
///
/// # Examples
///
/// ```
/// main();
/// ```
pub fn main() {
    println!("crate batch 1 starting");

    let abi_json = r#"
    {
        "functions": [
            {
                "name": "transfer",
                "inputs": [
                    {"name": "to", "type": "address"},
                    {"name": "value", "type": "uint256"}
                ],
                "outputs": []
            }
        ]
    }
    "#;
    run_1(abi_json);

    run_3("password", "$2y$12$XZ6J8vZc6Q1jz2X1Z5Q5eOe5eOe5eOe5eOe5eOe5eOe5eOe5eOe5eOe");
    run_4(&[1, 2, 3, 4]);
    run_5(&vec![0u8; 128], b"\n\x00\x00\x00\x04\x00\x00\x00\x00\x00");
    run_6("31 DEC 262143 23:59 -2359");
    run_7("x=yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy£");
    
    let s = b"; Test the division legalizttions.\ntest legalizer\n; See also legalize-div-traps.clif.\nset avoid_div_traps=0\ntarget x86_64\n\n; regex: V=v\\d+\n; regex: EBB=ebb\\d+\n\nfunction %udiv(i64, i64) -> i64 {\nebb0(v0: i64, v1: i64):\n    ; check: ebb0(\n    v2 = udiv v0, v1\n    ; nextln: $(hi=$V) = iconst.i64 0\n    ; nextln: $(d=$V), $(r=$V) = x86_udivmodx v0, $hi, v1\n    return v2\n    ; 28, i64) -> i64 {\nebb0(v0: i64, v1: i64):\n    ; check: ebb0(\n    v2 = srem v0, v1\n    ; nextln: $(fm1=$V) = ifcmp_imm v1, -1\n    ; nextln: brif eq $fm1, $(m1=$EBB)\n    ; check: $(hi=$V) = sshr_imm\n    ; nextln: $(d=$V), $(r=$V) = x86_sdivmodx v0, $hi, v1\n    ; nextln: jump $(done=$EBB)($r)\n    ; check: $m1:\n    ; nextln: $(zero=$V) = iconst.i64 0\n    ; nextln: jump $(done=$ x86_udivmodx v ; check: $done(v2: i64):\n    return v2\n    ; nextln: return v2\n}\n";
    run_8(s);
    
    run_9("#123456");
    run_10("color: red;");
    
    let d: Vec<u8> = vec![194, 133, 45, 127, 29, 127, 127];
    run_11(&d);
    
    run_12(&[0x02, 0x01, 0x01]);
    
    let data13 = b"\x03\x00\x00kk\x00\x00\x00\x00\x00\x00\x00.\x00\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff;\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\xff\x0a\xff";
    run_13(data13);
    
    run_14("3 + 4 * 2");
    
    let data15 = b"\x00\xfe\xf7\xf7\xf7\xf7\xf7\xf7\xb7\xf7\x00\x00\x02\x10\x00\xfc\x01\x00\x00\x00\x00\x00\x00\x00\x002222\x01\x00\x00\x0022222\xfc\x00\x00\x00\x00\x00\x00\x00222[[[[[21[[[[[[[[[[[[[[[[[[[[[[[222[[2222\x91\x91\x91\x91\x00\x00\x00\x00\xaa\xaa\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xb2\xb2\xb2\xb2\xb2\xb2\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xbe&\x00\x00\x00\x00\x00\x00\x00\xbez\x00\x01\x00\xd0\x00-\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x012\x94\x94\x94\x94\x94\x9422222222222222222222222222\x01\x00\x00\x0022222\xe1222222222221[[[[[[[[[[[[K\x1b[[[[[[[[[[[[\x00\x00\x00\x00\x00\x00\x00\x00\x012\x94\x94\x94\x94\x94\x94\xf7\xf7\xf7\xf7\xf73\x00\xaa\xaa\x11\x03\x00\x00\x002222222222222222222222\x00\x00\x00\x00\xaa\xaa\xe7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\x00\x00\x00\x00\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\xaa\xaa\xaa\x02\x00\x98\x00\x00\x00\x002222\x01\x00\x00\x0022222\xfc\x00\x00\x00\x00\x00\x00\x00222[[[[[21[[[[[[[[[[[[[[[[[[[[[[[222[[2222\x91\x91\x91\x91\x91\x91\xd4\x91\x91\x91\x91t\x912222222222222222\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\xaa\xaa\xaa\xaa\xaa\xaa\xaa\x00\x00\x01\x00\x00\x00\xaa\xaa\x9cU\xaa\xaa\x01\xaa2\xaa\xff\xff\xff\xff\xff\xff\xff\x02]\x00\x01\xff";
    run_15(data15);
    
    let data16 = b"fLaC\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00H\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\\";
    run_16(data16);
    
    let data17: &[u8] = &[102, 103, 98, 3, 102, 103, 98, 0, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 219, 216, 216, 216, 216, 216, 216, 216, 39, 39, 39, 39, 39, 32, 39, 39, 39, 39, 39, 39, 39, 39, 39, 10, 169, 247, 247, 247, 247];
    run_17(data17);
}
// *** main end

/// Demonstrates parsing a JSON ABI string.
///
/// # Examples
///
/// ```
/// run_1(r#"{"functions": []}"#);
/// ```
pub fn run_1(abi_json: &str) {
    // Convert the string to an iterator (by splitting into lines)
    let abi_lines = abi_json.lines();

    // Call `parse` with an iterator
    match JsonAbi::parse(abi_lines) {
        Ok(parsed_abi) => {},
        Err(err) => {},
    }
}

// fn run_2() {
//     let data = b"R0VUIGh0dHBzOi8vZXhhbXBsZS5jb20gSFRUUC8xLjEKSG9zdDrIgw0KDQo=";
//     let mut cursor = Cursor::new(data.as_ref());
//     async_h1::server::decode(&mut cursor);
// }


pub fn run_3(password: &str, hash: &str) {
    // bcrypt::verify
    match bcrypt::verify(password, hash) {
        Ok(matched) => {
            if matched {
                println!("Password matches the hash");
            } else {
                println!("Password does not match the hash");
            }
        }
        Err(err) => eprintln!("Error verifying password: {}", err),
    };
}

pub fn run_4(data: &[u8]) {
    match
        bincode_6::decode_from_slice::<u32, _>(
            data,
            bincode_6::config::Configuration::standard()
        )
    {
        Ok((value, _)) => println!("Decoded value from bincode_6: {}", value),
        Err(err) => eprintln!("Error decoding from bincode_6: {}", err),
    }

    match
        bincode_7::decode_from_slice::<u32, _>(
            data,
            bincode_7::config::Configuration::standard()
        )
    {
        Ok((value, _)) => println!("Decoded value from bincode_7: {}", value),
        Err(err) => eprintln!("Error decoding from bincode_7: {}", err),
    }
}

pub fn run_5(buf: &[u8], crash: &[u8]) {
    bson_10::decode_document(&mut Cursor::new(&buf[..]));
    bson_11::decode_document(&mut Cursor::new(&buf[..]));

    let mut reader = std::io::Cursor::new(crash);
    let _ = bson_12::Document::from_reader(&mut reader);
}

pub fn run_6(rfc2822_date: &str) {
    print!("chrono_16");
    chrono_16::DateTime::parse_from_rfc2822(rfc2822_date);
    chrono_17::DateTime::checked_add_days(chrono_17::Utc::now(), chrono_17::Days::new(1));
}

pub fn run_7(data: &str) {
    let c = cookie::Cookie::parse(data).expect("failed to parse cookie");

    let key = cookie::Key::from(&[0u8; 64]); // Fully qualified path

    let mut jar = cookie::CookieJar::new();

    let signed = jar.signed_mut(&key); // Fully qualified path
}

#[macro_use]
extern crate cranelift_reader;
use std::str;

pub fn run_8(s: &[u8]) {
    let s_str = str::from_utf8(s).unwrap_or("");
    cranelift_reader::parse_test(s_str);
}

pub fn run_9(color: &str) {
    // ! parsehex is private, parse gets to parsehex
    csscolorparser::parse(color);
}

pub fn run_10(input: &str) {
    let mut parser_input = cssparser::ParserInput::new(input);
    let mut parser = cssparser::Parser::new(&mut parser_input);

    // Handle the result properly
    match parser.next_including_whitespace_and_comments() {
        Ok(token) => println!("Parsed token: {:?}", token),
        Err(err) => println!("Parsing error: {:?}", err),
    }
}

use cursive::views::{ Dialog, TextView };
pub fn run_11(d: &[u8]) {
    let str = str::from_utf8(d);
    if let Ok(s) = str {
        // Creates the cursive root - required for every application.
        let mut siv = cursive::default();

        // Creates a dialog with a single "Quit" button
        siv.add_layer(
            Dialog::around(TextView::new(s))
                .title("Cursive")
                .button("Quit", |s| s.quit())
        );

        siv.run();
    } else {
        println!("not valid utf8");
    }
}

use der::Decodable;
pub fn run_12(der_data: &[u8]) {
    // ! decode_to_array is the function yet is private, we use decode instead

    // Handle the `Result` correctly
    let mut decoder = match der::Decoder::new(der_data) {
        Ok(decoder) => decoder, // Successfully created Decoder
        Err(err) => {
            eprintln!("Failed to create Decoder: {}", err);
            return;
        }
    };
    match der::asn1::Any::decode(&mut decoder) {
        Ok(decoded) => println!("Decoded successfully: {:?}", decoded),
        Err(err) => eprintln!("Error decoding: {}", err),
    }
}

pub fn run_13(data: &[u8]) {
    let _ = der_parser::parse_der(data);
}

pub fn run_14(s: &str) {
    let _ = exmex::parse_with_default_ops::<f64>(s).unwrap();
}

pub fn run_15(data: &[u8]) {
    // ! fatfs
    let storage = std::io::Cursor::new(data.to_vec());
    fatfs::FileSystem::new(storage, fatfs::FsOptions::new());
}

use flac::{ByteStream, Stream};

pub fn run_16(data: &[u8]) {
    // ! flac
    let s = Stream::<ByteStream>::from_buffer(data);
    if let Ok(mut stream) = s {
        let _ = stream.info();
        let _ = stream.metadata();
        let mut iter = stream.iter::<i8>();
        while iter.next().is_some() { }
    }
}

use flatgeobuf::*;
/// Demonstrates the usage of the `flatgeobuf` crate.
///
/// # Examples
///
/// ```
/// run_17(&[102, 103, 98, 3]);
/// ```
pub fn run_17(input: &[u8]) {
    let mut buf_reader = BufReader::new(Cursor::new(input));

    // ! spreedsheet line 36
    let mut fgb = match FgbReader::open(&mut buf_reader) {
        Ok(n) => n,
        Err(_) => return,
    };
    // ! spreedsheet line 37
    let _ = fgb.header();
}
