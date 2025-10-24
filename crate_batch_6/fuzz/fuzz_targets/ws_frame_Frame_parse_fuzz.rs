#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::io::Cursor;
use ws::Frame;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u8>,
}

fuzz_target!(|input: FuzzInput| {
    let mut cursor = Cursor::new(input.data);
    let _ = Frame::parse(&mut cursor);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: ws 
//  - Crate Link: None 
//  - Crate Version: 0.7.4 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: ws::frame::(Struct)Frame 
//  - Use Statement: None 
//  - Function Name: parse 
//  - Function Usage: fn run_14() {
//     println!("run 14");
// 
//     // ! crashing input
//     // let bytes = b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xfe\xfe\xfe\xfe\xfe\xfe\t\x01\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xb1\n".to_vec();
// 
//     let bytes = b"\xff\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe".to_vec();
//     let mut data = std::io::Cursor::new(bytes);
//     let _ =  ws::Frame::parse(&mut data);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut Cursor<Vec<u8, Global>>) -> Result<Option<Frame>, Error>,
//   type_fields: [alloc::Global, ws::Frame, ws::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut Cursor<Vec<u8, Global>>) -> Result<Option<Frame>, Error>,
//   type_fields: [alloc::Global, ws::Frame, ws::Error] 
// }
// Struct construction metadata: {
//   type_name: ws::Frame,
//   type_fields: [bool, bool, bool, bool, ws::OpCode, core::Option, alloc::Vec] 
// }
// Struct construction metadata: {
//   type_name: ws::OpCode,
//   type_fields: [Continue, Text, Binary, Close, Ping, Pong, Bad] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: alloc::Vec,
//   type_fields: [tinytemplate::Instruction, alloc::Global] 
// }
// Struct construction metadata: {
//   type_name: tinytemplate::Instruction,
//   type_fields: [Literal(&'template str), Value(Path<'template>), FormattedValue(Path<'template>, &'template str), Branch(Path<'template>, bool, usize), PushNamedContext(Path<'template>, &'template str), PushIterationContext(Path<'template>, &'template str), PopContext, Iterate(usize), Goto(usize), Call(&'template str, Path<'template>)] 
// }
// Struct construction metadata: {
//   type_name: ws::Error,
//   type_fields: [ws::Kind, alloc::Cow] 
// }
// Struct construction metadata: {
//   type_name: ws::Kind,
//   type_fields: [Internal, Capacity, Protocol, Encoding(Utf8Error), Io(io::Error), Http(httparse::Error), Queue(mio::channel::SendError<Command>), Timer(mio::timer::TimerError), Custom(Box<dyn StdError + Send + Sync>)] 
// }
// Struct construction metadata: {
//   type_name: alloc::Cow,
//   type_fields: [Borrowed(&'a B), Owned(<B as ToOwned>::Owned)] 
// }

