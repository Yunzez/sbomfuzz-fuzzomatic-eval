#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use cookie::CookieJar;
use cookie::Cookie;
use cookie::Key;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: String,
}

fuzz_target!(|input: FuzzInput| {
    if let Ok(cookie) = Cookie::parse(&input.data) {
        let key = Key::from(&[0u8; 64]);
        let mut jar = CookieJar::new();
        let _ = jar.signed_mut(&key);
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: cookie 
//  - Crate Link: None 
//  - Crate Version: 0.15.1 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: cookie::jar::(Struct)CookieJar 
//  - Use Statement: None 
//  - Function Name: signed_mut 
//  - Function Usage: fn run_7() {
//     let data = "x=yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy£";
// 
//     let c = cookie::Cookie::parse(data).expect("failed to parse cookie");
// 
//     let key = cookie::Key::from(&[0u8; 64]); // Fully qualified path
// 
//     let mut jar = cookie::CookieJar::new();
// 
//     let signed = jar.signed_mut(&key); // Fully qualified path
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&'a mut CookieJar, &Key) -> SignedJar<&'a mut CookieJar>,
//   type_fields: [cookie::CookieJar, cookie::Key] 
// }
// Struct construction metadata: {
//   type_name: fn(&'a mut CookieJar, &Key) -> SignedJar<&'a mut CookieJar>,
//   type_fields: [cookie::CookieJar, cookie::Key] 
// }
// Struct construction metadata: {
//   type_name: cookie::CookieJar,
//   type_fields: [std::HashSet, std::HashSet] 
// }
// Struct construction metadata: {
//   type_name: std::HashSet,
//   type_fields: [cookie::DeltaCookie, std::RandomState] 
// }
// Struct construction metadata: {
//   type_name: cookie::DeltaCookie,
//   type_fields: [cookie::Cookie, bool] 
// }
// Struct construction metadata: {
//   type_name: cookie::Cookie,
//   type_fields: [core::Option, cookie::CookieStr, cookie::CookieStr, core::Option, core::Option, core::Option, core::Option, core::Option, core::Option, core::Option] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: cookie::CookieStr,
//   type_fields: [Indexed(usize, usize), Concrete(Cow<'c, str>)] 
// }
// Struct construction metadata: {
//   type_name: std::RandomState,
//   type_fields: [u64, u64] 
// }
// Struct construction metadata: {
//   type_name: cookie::Key,
//   type_fields: [[u8; COMBINED_KEY_LENGTH]] 
// }

