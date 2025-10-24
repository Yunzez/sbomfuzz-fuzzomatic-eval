#![no_main]
use libfuzzer_sys::fuzz_target;
use ssh_parser::ssh::parse_ssh_packet;


fuzz_target!(|data: &[u8]| {
    // Catch potential panics to avoid fuzz target crashes
    let _ = std::panic::catch_unwind(|| {
        // Attempt parsing the SSH packet with the provided data
        let _ = parse_ssh_packet(data);
    }).unwrap_or_else(|e| {
        // Log the panic if it occurs
        eprintln!("Caught panic: {:?}", e);
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: ssh_parser 
//  - Crate Link: https://github.com/rusticata/ssh-parser 
//  - Crate Version: 0.1.0 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: ssh_parser::ssh 
//  - Use Statement: None 
//  - Function Name: parse_ssh_packet 
//  - Function Usage: fn run_14() {
//     println!("run 14");
// 
//     let data = b"\x00\x00\x00\x00\x00\x00\x00\x00";
// 
//     if
//         let Err(e) = std::panic::catch_unwind(|| {
//             let _ = ssh_parser::parse_ssh_packet(data);
//         })
//     {
//         eprintln!("Caught panic in run 14: {:?}", e);
//     }
// } 
//  - Parameters: initial function signature:No type_fields: fn(&[u8]) -> IResult<&[u8], (SshPacket<'_>, &[u8]), u32>

