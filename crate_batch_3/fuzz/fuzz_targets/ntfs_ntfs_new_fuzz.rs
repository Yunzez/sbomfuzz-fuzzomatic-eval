#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::io::Cursor;


#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let mut cursor = Cursor::new(input.data);
    if let Ok(mut fs) = ntfs::Ntfs::new(&mut cursor) {
        let _ = fs.read_upcase_table(&mut cursor);
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: ntfs 
//  - Crate Link: https://github.com/ColinFinck/ntfs 
//  - Crate Version: 0.1.0 
//  - From Crate: crate_batch_3 
//  - From Crate Link: unknown_website 
//  - Module Path: ntfs::ntfs::(Struct)Ntfs 
//  - Use Statement: None 
//  - Function Name: new 
//  - Function Usage: fn run_14() {
//     //!  crashing output
//     // let data = [
//     //     235, 82, 144, 78, 84, 70, 83, 32, 32, 32, 32, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 248, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 128, 0, 255, 222, 222, 94, 1, 94, 1, 222, 222,
//     //     222, 1, 0, 0, 0, 0, 0, 255, 7, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 8, 0, 112, 112, 121, 32, 97,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 8, 0, 0, 0, 0, 0, 0, 85, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 250,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 128,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 85, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     //     0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 250, 0, 0, 0, 0, 0, 0, 0, 0, 85,
//     //     170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 250, 0,
//     //     0, 0, 0, 0,
//     // ];
// 
//     let data = [
//         235, 82, 144, 78, 84, 70, 83, 32, 32, 32, 32, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 248, 0, 0, 0,
//     ];
//     // ? line 105
//     let mut data_2 = std::io::Cursor::new(data);
//     let _ = ntfs::Ntfs::new(&mut data_2);
// 
//     // ? line 106
//     let mut mutdata = [
//         235, 82, 144, 78, 84, 70, 83, 32, 32, 0, 0, 0, 0, 0, 0, 128, 32, 128, 0, 255, 15, 0, 0, 0,
//         0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 255, 7, 0, 0, 0, 0, 0, 0, 149, 0, 0, 0, 8, 0, 0, 0, 120,
//         183, 16, 124, 224, 39, 74, 127, 0, 0, 0, 0, 14, 31, 190, 113, 124, 172, 34, 192, 116, 11, 86,
//         180, 14, 187, 7, 0, 205, 16, 94, 235, 240, 50, 228, 205, 22, 205, 25, 235, 254, 84, 104,
//         105, 115, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 98, 111, 111, 116, 97, 98, 108, 101,
//         32, 100, 105, 115, 107, 46, 32, 80, 50, 101, 97, 115, 101, 32, 105, 110, 115, 101, 114, 116,
//         32, 97, 32, 98, 111, 111, 116, 97, 98, 108, 101, 32, 102, 108, 111, 112, 112, 121, 32, 97,
//         110, 100, 13, 10, 112, 114, 101, 115, 115, 32, 97, 110, 121, 32, 107, 101, 121, 32, 116, 111,
//         32, 116, 114, 121, 32, 97, 103, 97, 105, 110, 32, 97, 110, 121, 32, 107, 101, 121, 32, 116,
//         111, 32, 116, 114, 121, 32, 97, 103, 97, 105, 110, 32, 46, 46, 46, 32, 13, 10, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 2,
//         183, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 170,
//     ];
// 
//     let mut data = std::io::Cursor::new(mutdata);
//     if let Ok(mut fs) = ntfs::Ntfs::new(&mut data) {
//         if let Err(e) = fs.read_upcase_table(&mut data) {
//             eprintln!("Failed to read upcase table: {}", e);
//         }
//     } else {
//         eprintln!("Failed to create NTFS filesystem");
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut T) -> Result<Ntfs, NtfsError>,
//   type_fields: [T, ntfs::Ntfs, ntfs::NtfsError] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut T) -> Result<Ntfs, NtfsError>,
//   type_fields: [T, ntfs::Ntfs, ntfs::NtfsError] 
// }
// Struct construction metadata: {
//   type_name: ntfs::Ntfs,
//   type_fields: [u32, u16, u64, u64, u32, u64, core::Option] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: ntfs::NtfsError,
//   type_fields: [AttributeNotFound, AttributeOfDifferentType, BufferTooSmall, InvalidAttributeNameLength, InvalidAttributeNameOffset, InvalidByteCountInDataRunHeader, InvalidClusterCount, InvalidFileAllocatedSize, InvalidFileRecordNumber, InvalidFileSignature, InvalidFileUsedSize, InvalidIndexAllocatedSize, InvalidIndexEntryDataRange, InvalidIndexEntrySize, InvalidIndexRootEntriesOffset, InvalidIndexRootUsedSize, InvalidIndexSignature, InvalidIndexUsedSize, InvalidResidentAttributeValueLength, InvalidResidentAttributeValueOffset, InvalidRecordSizeInfo, InvalidStructuredValueSize, InvalidTime, InvalidTwoByteSignature, InvalidUpcaseTableSize, InvalidVcnInDataRunHeader, Io(binread::io::Error), LcnTooBig, MissingIndexAllocation, NotADirectory, UnexpectedAttributeListAttribute, UnexpectedNonResidentAttribute, UnexpectedResidentAttribute, UnsupportedAttributeType, UnsupportedClusterSize, UnsupportedFileNamespace, UpdateSequenceArrayExceedsRecordSize, UpdateSequenceNumberMismatch, VcnMismatchInIndexAllocation, VcnOutOfBoundsInIndexAllocation, VcnTooBig] 
// }

