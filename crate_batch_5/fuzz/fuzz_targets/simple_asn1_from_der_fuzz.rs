#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use simple_asn1::from_der;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a [u8],
}

fuzz_target!(|input: FuzzInput| {
    // Use `catch_unwind` to handle any potential panics safely
    let _ = std::panic::catch_unwind(|| {
        let _ = from_der(input.data);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: simple_asn1 
//  - Crate Link: unknown_website 
//  - Crate Version: 0.3.1 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: simple_asn1 
//  - Use Statement: None 
//  - Function Name: from_der 
//  - Function Usage: fn run_8() {
//     let data = &[0x30, 0x0a, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02, 0x02, 0x01];
// 
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             let _ = simple_asn1::from_der(data); // should panic
//         })
//     {
//         eprintln!("Caught panic in run 8: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&[u8]) -> Result<Vec<ASN1Block, Global>, ASN1DecodeErr>,
//   type_fields: [simple_asn1::ASN1Block, alloc::Global, simple_asn1::ASN1DecodeErr] 
// }
// Struct construction metadata: {
//   type_name: fn(&[u8]) -> Result<Vec<ASN1Block, Global>, ASN1DecodeErr>,
//   type_fields: [simple_asn1::ASN1Block, alloc::Global, simple_asn1::ASN1DecodeErr] 
// }
// Struct construction metadata: {
//   type_name: simple_asn1::ASN1Block,
//   type_fields: [Boolean(usize, bool), Integer(usize, BigInt), BitString(usize, usize, Vec<u8>), OctetString(usize, Vec<u8>), Null(usize), ObjectIdentifier(usize, OID), UTF8String(usize, String), PrintableString(usize, String), TeletexString(usize, String), IA5String(usize, String), UTCTime(usize, DateTime<Utc>), GeneralizedTime(usize, DateTime<Utc>), UniversalString(usize, String), BMPString(usize, String), Sequence(usize, Vec<ASN1Block>), Set(usize, Vec<ASN1Block>), Explicit(ASN1Class, usize, BigUint, Box<ASN1Block>), Unknown(ASN1Class, bool, usize, BigUint, Vec<u8>)] 
// }
// Struct construction metadata: {
//   type_name: simple_asn1::ASN1DecodeErr,
//   type_fields: [EmptyBuffer, BadBooleanLength(usize), LengthTooLarge(usize), UTF8DecodeFailure(Utf8Error), PrintableStringDecodeFailure, InvalidDateValue(String)] 
// }

