#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use serde_yaml::from_str;
use serde_yaml::to_string;
use serde_yaml::Value;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    yaml_string: String,
}

fuzz_target!(|data: FuzzInput| {
    if let Ok(parsed_value) = from_str::<Value>(&data.yaml_string) {
        if let Ok(serialized_yaml) = to_string(&parsed_value) {
            let _ = from_str::<Value>(&serialized_yaml);
        }
    }
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: serde_yaml 
//  - Crate Link: unknown_website 
//  - Crate Version: 0.8.26 
//  - From Crate: crate_batch_5 
//  - From Crate Link: unknown_website 
//  - Module Path: serde_yaml::de 
//  - Use Statement: None 
//  - Function Name: from_str 
//  - Function Usage: fn run_7() {
//     // ? line 159
// 
//     let data = b"invalid yaml data";
//     match serde_yaml::from_slice::<serde_yaml::Value>(data) {
//         Ok(value) => println!("Parsed YAML successfully: {:?}", value),
//         Err(err) => eprintln!("Failed to parse YAML: {}", err),
//     }
// 
//     // ? line 160
//     // Step 1: Start with a YAML string containing "50."
//     let yaml_str = "50.";
//     let deserialized: Number = serde_yaml::from_str(yaml_str).unwrap();
// 
//     println!("Deserialized from YAML (50.): {:?}", deserialized);
// 
//     // Step 2: Serialize it back to YAML
//     let serialized_yaml = serde_yaml::to_string(&deserialized).unwrap();
//     println!("Serialized to YAML: {}", serialized_yaml);
// 
//     // Step 3: Deserialize again and check type
//     let roundtrip: Number = serde_yaml::from_str(&serialized_yaml).unwrap();
//     println!("Deserialized again: {:?}", roundtrip);
// 
//     assert_eq!(deserialized, roundtrip, "Roundtrip failed!");
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&str) -> Result<T, Error>,
//   type_fields: [T, serde_yaml::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&str) -> Result<T, Error>,
//   type_fields: [T, serde_yaml::Error] 
// }
// Struct construction metadata: {
//   type_name: serde_yaml::Error,
//   type_fields: [alloc::Box] 
// }
// Struct construction metadata: {
//   type_name: alloc::Box,
//   type_fields: [serde_yaml::ErrorImpl, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: serde_yaml::ErrorImpl,
//   type_fields: [Message(String, Option<Pos>), Emit(emitter::EmitError), Scan(scanner::ScanError), Io(io::Error), Utf8(str::Utf8Error), FromUtf8(string::FromUtf8Error), EndOfStream, MoreThanOneDocument, RecursionLimitExceeded, Shared(Arc<ErrorImpl>)] 
// }

