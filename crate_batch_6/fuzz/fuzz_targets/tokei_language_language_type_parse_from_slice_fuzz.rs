#![no_main]
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;
use std::panic;
use tokei::{Config, LanguageType, LanguageType::*};

#[derive(Arbitrary, Debug)]
struct FuzzInput<'a> {
    data: &'a [u8],
    treat_doc_strings_as_comments: bool,
}

fuzz_target!(|input: FuzzInput| {
    let language = Vue; // We are using Vue as indicated by the function usage example
    
    // Config creation based on the function usage
    let config = Config {
        treat_doc_strings_as_comments: Some(input.treat_doc_strings_as_comments),
        ..Config::default()
    };

    // Using catch_unwind to prevent panics from crashing the fuzzer
    let _ = panic::catch_unwind(|| {
        language.parse_from_slice(input.data, &config)
    });
});// 
//  
//  Metadata
// Function Info: 
//  - Macro: false 
//  - Crate: tokei 
//  - Crate Link: https://tokei.rs 
//  - Crate Version: 12.1.2 
//  - From Crate: crate_batch_6 
//  - From Crate Link: unknown_website 
//  - Module Path: tokei::language::language_type::(Enum)LanguageType 
//  - Use Statement: None 
//  - Function Name: parse_from_slice 
//  - Function Usage: fn run_3() {
//     println!("run 3");
//     // ? line 196 197 are same function (Tokei)
//     // The input content causing the panic.
//     let buggy_input = b"<script>\ra </script>";
//     let language = LanguageType::Vue;
//     let config = &(Config {
//         treat_doc_strings_as_comments: Some(true),
//         ..Config::default()
//     });
//     
// 
//     let result = panic::catch_unwind(|| { language.parse_from_slice(buggy_input, config) });
// 
//     match result {
//         Ok(_) => println!("Parsed successfully"),
//         Err(_) => eprintln!("❌ Caught panic in `parse_from_slice`!"),
//     }
// } 
//  - Parameters: initial function signature:{
//   type_name: fn(LanguageType, A, &Config) -> CodeStats,
//   type_fields: [tokei::LanguageType, A, tokei::Config, tokei::CodeStats] 
// }
// Struct construction metadata: {
//   type_name: fn(LanguageType, A, &Config) -> CodeStats,
//   type_fields: [tokei::LanguageType, A, tokei::Config, tokei::CodeStats] 
// }
// Struct construction metadata: {
//   type_name: tokei::LanguageType,
//   type_fields: [ABNF, Abap, ActionScript, Ada, Agda, Alex, Alloy, Arduino, AsciiDoc, Asn1, Asp, AspNet, Assembly, AssemblyGAS, AutoHotKey, Autoconf, Automake, Bash, Batch, Bean, BrightScript, C, CHeader, CMake, CSharp, CShell, Cabal, Cassius, Ceylon, Clojure, ClojureC, ClojureScript, Cobol, CodeQL, CoffeeScript, Cogent, ColdFusion, ColdFusionScript, Coq, Cpp, CppHeader, Crystal, Css, D, Daml, Dart, DeviceTree, Dhall, Dockerfile, DotNetResource, DreamMaker, Dust, Edn, Elisp, Elixir, Elm, Elvish, EmacsDevEnv, Emojicode, Erlang, FEN, FSharp, Fish, FlatBuffers, Forth, FortranLegacy, FortranModern, FreeMarker, Fstar, Futhark, GDB, GdScript, Gherkin, Gleam, Glsl, Gml, Go, Gohtml, Graphql, Groovy, Gwion, Hamlet, Handlebars, Happy, Haskell, Haxe, Hcl, Headache, Hex, Hlsl, HolyC, Html, Idris, Ini, IntelHex, Isabelle, Jai, Java, JavaScript, Json, Jsonnet, Jsx, Julia, Julius, Jupyter, K, KakouneScript, Kotlin, KvLanguage, LLVM, Lean, Less, LinkerScript, Liquid, Lisp, LiveScript, Logtalk, Lua, Lucius, Madlang, Makefile, Markdown, Meson, Mint, ModuleDef, MoonScript, MsBuild, Mustache, Nim, Nix, NotQuitePerl, OCaml, ObjectiveC, ObjectiveCpp, Odin, OpenType, Org, Oz, PSL, Pan, Pascal, Perl, Perl6, Pest, Php, Polly, Pony, PostCss, PowerShell, Processing, Prolog, Protobuf, Pug, PureScript, Python, Q, Qcl, Qml, R, RON, RPMSpecfile, Racket, Rakefile, Razor, ReStructuredText, Renpy, Ruby, RubyHtml, Rust, SRecode, Sass, Scala, Scheme, Scons, Sh, Sml, Solidity, SpecmanE, Spice, Sql, Stan, Stratego, Stylus, Svelte, Svg, Swift, Swig, SystemVerilog, Tcl, Tera, Tex, Text, Thrift, Toml, Tsx, Ttcn, Twig, TypeScript, UnrealDeveloperMarkdown, UnrealPlugin, UnrealProject, UnrealScript, UnrealShader, UnrealShaderHeader, UrWeb, UrWebProject, VB6, VBScript, Vala, Velocity, Verilog, VerilogArgsFile, Vhdl, VimScript, VisualBasic, VisualStudioProject, VisualStudioSolution, Vue, WebAssembly, Wolfram, XSL, Xaml, XcodeConfig, Xml, Xtend, Yaml, Zig, Zsh] 
// }
// Struct construction metadata: {
//   type_name: tokei::Config,
//   type_fields: [core::Option, core::Option, core::Option, core::Option, core::Option, core::Option, core::Option, core::Option, core::Option] 
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)] 
// }
// Struct construction metadata: {
//   type_name: tokei::CodeStats,
//   type_fields: [usize, usize, usize, alloc::BTreeMap] 
// }
// Struct construction metadata: {
//   type_name: alloc::BTreeMap,
//   type_fields: [tokei::LanguageType, tokei::CodeStats, alloc::Global] 
// }

