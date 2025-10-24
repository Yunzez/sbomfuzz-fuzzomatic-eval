#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use bincode_6::{config::Configuration, decode_from_slice, error::DecodeError};

// Define a fuzzing input structure
#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    bytes: &'a [u8],
}

fuzz_target!(|data: FuzzInput| {
    let config = Configuration::standard();

    // Attempt to decode the fuzzed bytes with the standard configuration
    match decode_from_slice::<u32, _>(data.bytes, config) {
        Ok((_, _)) => {},  // Successfully decoded
        Err(DecodeError::UnexpectedEnd) => {},
        Err(DecodeError::LimitExceeded) => {},
        Err(DecodeError::Utf8(_)) => {},
        Err(DecodeError::InvalidCharEncoding(_)) => {},
        Err(DecodeError::InvalidBooleanValue(_)) => {},
        Err(DecodeError::OtherString(_)) => {},
        Err(_) => {},  // Handle other errors gracefully
    };
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: bincode 
//  - Crate Link: None 
//  - Crate Version: 2.0.0-beta.0 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: bincode 
//  - Use Statement: None 
//  - Function Name: decode_from_slice 
//  - Function Usage: fn run_4() {
//     match
//         bincode_6::decode_from_slice::<u32, _>(
//             &[1, 2, 3, 4],
//             bincode_6::config::Configuration::standard()
//         )
//     {
//         Ok((value, _)) => println!("Decoded value from bincode_6: {}", value),
//         Err(err) => eprintln!("Error decoding from bincode_6: {}", err),
//     }
// 
//     match
//         bincode_7::decode_from_slice::<u32, _>(
//             &[1, 2, 3, 4],
//             bincode_7::config::Configuration::standard()
//         )
//     {
//         Ok((value, _)) => println!("Decoded value from bincode_7: {}", value),
//         Err(err) => eprintln!("Error decoding from bincode_7: {}", err),
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a [u8], C) -> Result<(D, usize), DecodeError>,
//   type_fields: [C, D, bincode::DecodeError] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a [u8], C) -> Result<(D, usize), DecodeError>,
//   type_fields: [C, D, bincode::DecodeError] 
// }
// Struct construction metadata: {
//   type_name: bincode::DecodeError,
//   type_fields: [UnexpectedEnd, LimitExceeded, InvalidIntegerType, NonZeroTypeIsZero, UnexpectedVariant, Utf8(core::str::Utf8Error), InvalidCharEncoding([u8; {const}]), InvalidBooleanValue(u8), ArrayLengthMismatch, EmptyEnum, InvalidDuration, CStrNulError, OtherString(alloc::string::String)] 
// }

