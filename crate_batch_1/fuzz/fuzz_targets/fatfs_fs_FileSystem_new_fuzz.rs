#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::io::Cursor;
use fatfs::{FileSystem, FsOptions};

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let storage = Cursor::new(input.data);
    let _ = FileSystem::new(storage, FsOptions::new());
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: fatfs 
//  - Crate Link: None 
//  - Crate Version: 0.3.6 
//  - From Crate: crate_batch_1 
//  - From Crate Link: unknown_website 
//  - Module Path: fatfs::fs::(Struct)FileSystem 
//  - Use Statement: None 
//  - Function Name: new 
//  - Function Usage: fn run_15() {
// 
//     // ! fatfs
//     let data = b"\x00\xfe\xf7\xf7\xf7\xf7\xf7\xf7\xb7\xf7\x00\x00\x02\x10\x00\xfc\x01\x00\x00\x00\x00\x00\x00\x00\x002222\x01\x00\x00\x0022222\xfc\x00\x00\x00\x00\x00\x00\x00222[[[[[21[[[[[[[[[[[[[[[[[[[[[[[222[[2222\x91\x91\x91\x91\x00\x00\x00\x00\xaa\xaa\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xb2\xb2\xb2\xb2\xb2\xb2\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xbe&\x00\x00\x00\x00\x00\x00\x00\xbez\x00\x01\x00\xd0\x00-\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x012\x94\x94\x94\x94\x94\x9422222222222222222222222222\x01\x00\x00\x0022222\xe1222222222221[[[[[[[[[[[[K\x1b[[[[[[[[[[[[\x00\x00\x00\x00\x00\x00\x00\x00\x012\x94\x94\x94\x94\x94\x94\xf7\xf7\xf7\xf7\xf73\x00\xaa\xaa\x11\x03\x00\x00\x002222222222222222222222\x00\x00\x00\x00\xaa\xaa\xe7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\x00\x00\x00\x00\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\xaa\xaa\xaa\x02\x00\x98\x00\x00\x00\x002222\x01\x00\x00\x0022222\xfc\x00\x00\x00\x00\x00\x00\x00222[[[[[21[[[[[[[[[[[[[[[[[[[[[[[222[[2222\x91\x91\x91\x91\x91\x91\xd4\x91\x91\x91\x91t\x912222222222222222\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\xaa\xaa\xaa\xaa\xaa\xaa\xaa\x00\x00\x01\x00\x00\x00\xaa\xaa\x9cU\xaa\xaa\x01\xaa2\xaa\xff\xff\xff\xff\xff\xff\xff\x02]\x00\x01\xff";
// 
//     let storage = std::io::Cursor::new(data.to_vec());
//     fatfs::FileSystem::new(storage, fatfs::FsOptions::new());
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(T, FsOptions) -> Result<FileSystem<T>, Error>,
//   type_fields: [T, fatfs::FsOptions, std::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(T, FsOptions) -> Result<FileSystem<T>, Error>,
//   type_fields: [T, fatfs::FsOptions, std::Error] 
// }
// Struct construction metadata: {
//   type_name: fatfs::FsOptions,
//   type_fields: [bool, &'static (dyn OemCpConverter + 'static), &'static (dyn TimeProvider + 'static)] 
// }
// Struct construction metadata: {
//   type_name: std::Error,
//   type_fields: [std::Repr] 
// }
// Struct construction metadata: {
//   type_name: std::Repr,
//   type_fields: [core::NonNull, core::PhantomData] 
// }
// Struct construction metadata: {
//   type_name: core::NonNull,
//   type_fields: [*const str] 
// }

