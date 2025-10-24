#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
extern crate der;
use der::Decoder;
use der::Decodable;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    der_data: &[u8],
}

fuzz_target!(|input: &[u8]| {
    // Attempt to create a Decoder with the provided data
    if let Ok(mut decoder) = Decoder::new(input.der_data) {
        // Test decoding with the decoder
        let _ = der::asn1::Any::decode(&mut decoder);
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: der 
//  - Crate Link: None 
//  - Crate Version: 0.6.0-pre.1 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: der::decoder::(Struct)Decoder 
//  - Use Statement: None 
//  - Function Name: new 
//  - Function Usage: fn run_12() {
//     // ! decode_to_array is the function yet is private, we use decode instead
//     let der_data = &[0x02, 0x01, 0x01]; // DER encoding of an integer (1)
// 
//     // Handle the `Result` correctly
//     let mut decoder = match der::Decoder::new(der_data) {
//         Ok(decoder) => decoder, // Successfully created Decoder
//         Err(err) => {
//             eprintln!("Failed to create Decoder: {}", err);
//             return;
//         }
//     };
//     match der::asn1::Any::decode(&mut decoder) {
//         Ok(decoded) => println!("Decoded successfully: {:?}", decoded),
//         Err(err) => eprintln!("Error decoding: {}", err),
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a [u8]) -> Result<Decoder<'a>, Error>,
//   type_fields: [der::Decoder, der::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a [u8]) -> Result<Decoder<'a>, Error>,
//   type_fields: [der::Decoder, der::Error] 
// }
// Struct construction metadata: {
//   type_name: der::Decoder,
//   type_fields: [core::Option, der::Length, der::Length] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: der::Length,
//   type_fields: [u32] 
// }
// Struct construction metadata: {
//   type_name: der::Error,
//   type_fields: [der::ErrorKind, core::Option] 
// }
// Struct construction metadata: {
//   type_name: der::ErrorKind,
//   type_fields: [DateTime, Failed, Incomplete, Length, Noncanonical, OidMalformed, SetOrdering, Overflow, Overlength, TagModeUnknown, TagNumberInvalid, TagUnexpected, TagUnknown, TrailingData, Utf8(Utf8Error), Value] 
// }

