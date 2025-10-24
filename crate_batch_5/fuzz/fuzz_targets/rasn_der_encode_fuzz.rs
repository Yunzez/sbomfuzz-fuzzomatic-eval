#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = rasn::der::decode::<rasn::types::Open>(data) {
        let encoded = rasn::der::encode(&value).unwrap_or_default();
        let _ = rasn::der::decode::<rasn::types::Open>(&encoded);
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: rasn 
//  - Crate Link: None 
//  - Crate Version: 0.9.5 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: rasn::der 
//  - Use Statement: None 
//  - Function Name: encode 
//  - Function Usage: fn run_1() {
//     // ! crashing input
//     let data = [24, 19, 43, 53, 49, 54, 49, 53, 32, 32, 48, 53, 50, 52, 48, 57, 52, 48, 50, 48, 90];
// 
//     // Attempt to decode, handle errors
//     let value = match rasn::der::decode::<rasn::types::Open>(&data) {
//         Ok(v) => v,
//         Err(e) => {
//             eprintln!("Decoding failed: {:?}", e);
//             return; // Stop execution here if decoding fails
//         }
//     };
// 
//     // Try encoding & decoding again
//     let encoded = rasn::der::encode(&value).unwrap();
// 
//     match rasn::der::decode::<rasn::types::Open>(&encoded) {
//         Ok(_) => println!("Decoding succeeded!"),
//         Err(e) => eprintln!("Decoding failed after encoding: {:?}", e),
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&T) -> Result<Vec<u8, Global>, Error>,
//   type_fields: [T, alloc::Global, rasn::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&T) -> Result<Vec<u8, Global>, Error>,
//   type_fields: [T, alloc::Global, rasn::Error] 
// }
// Struct construction metadata: {
//   type_name: rasn::Error,
//   type_fields: [Incomplete, ConstructedEncodingNotAllowed, IndefiniteLengthNotAllowed, InvalidBool, InvalidObjectIdentifier, InvalidUtf8, InvalidDate, Parser, UnexpectedExtraData, MismatchedTag, MismatchedLength, ExceedsMaxLength, IntegerOverflow, InvalidBitString, MissingField, NoValidChoice, FieldError, Custom] 
// }

