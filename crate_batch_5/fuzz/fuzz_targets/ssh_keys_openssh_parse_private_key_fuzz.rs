#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use ssh_keys::openssh;

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a str,
}

fuzz_target!(|input: FuzzInput| {
    let _ = std::panic::catch_unwind(|| {
        let _ = openssh::parse_private_key(input.data);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: ssh_keys 
//  - Crate Link: http://github.com/tailhook/ssh-keys 
//  - Crate Version: 0.1.0 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: ssh_keys::openssh 
//  - Use Statement: None 
//  - Function Name: parse_private_key 
//  - Function Usage: fn run_13() {
//     println!("run 13");
//     let data =
//         "-----BEGIN OPENSSH PRIVATE KEY------END OPENSSH PRIVATE KEY-----ENSSH PRIVAPRIVATE KEY-----\x00\x00\x00\x01\x00";
// 
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             ssh_keys::openssh::parse_private_key(data);
//         })
//     {
//         eprintln!("Caught panic in run 13: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Result<Vec<PrivateKey, Global>, Error>,
//   type_fields: [ssh_keys::PrivateKey, alloc::Global, ssh_keys::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Result<Vec<PrivateKey, Global>, Error>,
//   type_fields: [ssh_keys::PrivateKey, alloc::Global, ssh_keys::Error] 
// }
// Struct construction metadata: {
//   type_name: ssh_keys::PrivateKey,
//   type_fields: [Rsa, Ed25519([u8; {const}])] 
// }
// Struct construction metadata: {
//   type_name: ssh_keys::Error,
//   type_fields: [InvalidFormat, UnsupportedType(String), Encrypted, __Nonexhaustive] 
// }

