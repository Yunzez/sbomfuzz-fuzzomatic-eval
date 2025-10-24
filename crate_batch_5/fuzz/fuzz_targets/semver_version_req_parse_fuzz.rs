#![no_main]
use libfuzzer_sys::fuzz_target;
use semver::VersionReq;
use arbitrary::Arbitrary;

fuzz_target!(|data: &str| {
    // Attempt to parse the version requirement
    let _ = VersionReq::parse(data);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: semver 
//  - Crate Link: https://docs.rs/crate/semver/ 
//  - Crate Version: 0.9.0 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: semver::version_req::(Struct)VersionReq 
//  - Use Statement: None 
//  - Function Name: parse 
//  - Function Usage: fn run_6() {
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             semver::VersionReq::parse("8.*.7").unwrap(); // should panic
//         })
//     {
//         eprintln!("Caught panic in run 6: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Result<VersionReq, ReqParseError>,
//   type_fields: [semver::VersionReq, semver::ReqParseError] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Result<VersionReq, ReqParseError>,
//   type_fields: [semver::VersionReq, semver::ReqParseError] 
// }
// Struct construction metadata: {
//   type_name: semver::VersionReq,
//   type_fields: [alloc::Vec] 
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
//   type_fields: [*const u8] 
// }
// Struct construction metadata: {
//   type_name: core::UsizeNoHighBit,
//   type_fields: [usize] 
// }
// Struct construction metadata: {
//   type_name: semver::ReqParseError,
//   type_fields: [InvalidVersionRequirement, OpAlreadySet, InvalidSigil, VersionComponentsMustBeNumeric, InvalidIdentifier, MajorVersionRequired, UnimplementedVersionRequirement, DeprecatedVersionRequirement(VersionReq)] 
// }

