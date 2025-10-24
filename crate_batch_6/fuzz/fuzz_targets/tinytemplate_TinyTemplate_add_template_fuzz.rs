#![no_main]
use libfuzzer_sys::fuzz_target;
use tinytemplate::TinyTemplate;

fuzz_target!(|data: &[u8]| {
    let template_str = match std::str::from_utf8(data) {
        Ok(v) => v,
        Err(_) => return,
    };

    let mut tpl = TinyTemplate::new();
    let _ = tpl.add_template("fuzz_template", template_str);
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: tinytemplate 
//  - Crate Link: None 
//  - Crate Version: 1.2.1 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: tinytemplate::(Struct)TinyTemplate 
//  - Use Statement: None 
//  - Function Name: add_template 
//  - Function Usage: fn run_1() {
//     println!("run 1");
//     // ! crashing input
//     // let data = "{#}";
//     let data = "{}";
// 
//     let mut tpl = tinytemplate::TinyTemplate::new();
// 
//     let _ = tpl.add_template("template", data);
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(&mut TinyTemplate<'template>, &'template str, &'template str) -> Result<(), Error>,
//   type_fields: [tinytemplate::TinyTemplate, tinytemplate::Error] 
// }
// Struct construction metadata: {
//   type_name: fn(&mut TinyTemplate<'template>, &'template str, &'template str) -> Result<(), Error>,
//   type_fields: [tinytemplate::TinyTemplate, tinytemplate::Error] 
// }
// Struct construction metadata: {
//   type_name: tinytemplate::TinyTemplate,
//   type_fields: [std::HashMap, std::HashMap, &'template (dyn Fn(&Value, &mut String) -> Result<(), Error> + 'static)] 
// }
// Struct construction metadata: {
//   type_name: std::HashMap,
//   type_fields: [tinytemplate::Template, std::RandomState] 
// }
// Struct construction metadata: {
//   type_name: tinytemplate::Template,
//   type_fields: [&'template str, alloc::Vec, usize] 
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
//   type_name: std::RandomState,
//   type_fields: [u64, u64] 
// }
// Struct construction metadata: {
//   type_name: tinytemplate::Error,
//   type_fields: [ParseError, RenderError, SerdeError, GenericError, StdFormatError, CalledTemplateError, CalledFormatterError, __NonExhaustive] 
// }

