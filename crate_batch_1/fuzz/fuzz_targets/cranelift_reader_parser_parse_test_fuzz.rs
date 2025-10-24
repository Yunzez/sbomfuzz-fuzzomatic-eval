#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
extern crate cranelift_reader;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    input_str: String,
}

fuzz_target!(|data: FuzzInput| {
    // Attempt to parse the input string using parse_test
    let _ = cranelift_reader::parse_test(&data.input_str);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: cranelift_reader 
//  - Crate Link: None 
//  - Crate Version: 0.17.0 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: cranelift_reader::parser 
//  - Use Statement: None 
//  - Function Name: parse_test 
//  - Function Usage: fn run_8() {
//     let s = b"; Test the division legalizttions.\ntest legalizer\n; See also legalize-div-traps.clif.\nset avoid_div_traps=0\ntarget x86_64\n\n; regex: V=v\\d+\n; regex: EBB=ebb\\d+\n\nfunction %udiv(i64, i64) -> i64 {\nebb0(v0: i64, v1: i64):\n    ; check: ebb0(\n    v2 = udiv v0, v1\n    ; nextln: $(hi=$V) = iconst.i64 0\n    ; nextln: $(d=$V), $(r=$V) = x86_udivmodx v0, $hi, v1\n    return v2\n    ; 28, i64) -> i64 {\nebb0(v0: i64, v1: i64):\n    ; check: ebb0(\n    v2 = srem v0, v1\n    ; nextln: $(fm1=$V) = ifcmp_imm v1, -1\n    ; nextln: brif eq $fm1, $(m1=$EBB)\n    ; check: $(hi=$V) = sshr_imm\n    ; nextln: $(d=$V), $(r=$V) = x86_sdivmodx v0, $hi, v1\n    ; nextln: jump $(done=$EBB)($r)\n    ; check: $m1:\n    ; nextln: $(zero=$V) = iconst.i64 0\n    ; nextln: jump $(done=$ x86_udivmodx v ; check: $done(v2: i64):\n    return v2\n    ; nextln: return v2\n}\n";
// 
//     let s_str = std::str::from_utf8(s).expect("Invalid UTF-8 sequence");
//     cranelift_reader::parse_test(s_str);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Result<TestFile<'_>, ParseError>,
//   type_fields: [cranelift_reader::TestFile, cranelift_reader::ParseError] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Result<TestFile<'_>, ParseError>,
//   type_fields: [cranelift_reader::TestFile, cranelift_reader::ParseError] 
// }
// Struct construction metadata: {
//   type_name: cranelift_reader::TestFile,
//   type_fields: [alloc::Vec, cranelift_reader::IsaSpec, alloc::Vec, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [alloc::RawVec, usize] 
// }
// Struct construction metadata: {
//   type_name: alloc::RawVec,
//   type_fields: [alloc::RawVecInner, core::PhantomData] 
// }
// Struct construction metadata: {
//   type_name: alloc::RawVecInner,
//   type_fields: [core::Unique, core::UsizeNoHighBit, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: core::Unique,
//   type_fields: [core::NonNull, core::PhantomData] 
// }
// Struct construction metadata: {
//   type_name: core::NonNull,
//   type_fields: [*const str] 
// }
// Struct construction metadata: {
//   type_name: core::UsizeNoHighBit,
//   type_fields: [usize] 
// }
// Struct construction metadata: {
//   type_name: cranelift_reader::IsaSpec,
//   type_fields: [None(Flags), Some(Vec<Box<TargetIsa>>)] 
// }
// Struct construction metadata: {
//   type_name: cranelift_reader::ParseError,
//   type_fields: [cranelift_reader::Location, alloc::String] 
// }
// Struct construction metadata: {
//   type_name: cranelift_reader::Location,
//   type_fields: [usize] 
// }
// Struct construction metadata: {
//   type_name: alloc::String,
//   type_fields: [alloc::Vec] 
// }

