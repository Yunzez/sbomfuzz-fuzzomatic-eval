#![no_main]
use libfuzzer_sys::fuzz_target;
use cursive::{Cursive, views::Dialog, views::TextView};
use cursive::CursiveRunnable;

fuzz_target!(|data: Vec<u8>| {
    if let Ok(s) = std::str::from_utf8(&data) {
        let mut siv = Cursive::default();
        siv.add_layer(
            cursive::views::Dialog
                ::around(cursive::views::TextView::new(s))
                .title("Cursive")
                .button("Quit", |s| s.quit())
        );

        // If we have CursiveRunnable, we would invoke run method like this
        siv.run();
    }
}); //
//
//  Metadata
// Function Info:
//  - Macro: false
//  - Crate: cursive
//  - Crate Link: None
//  - Crate Version: 0.21.1
//  - From Crate: crate_batch_1
//  - From Crate Link: unknown_website
//  - Module Path: cursive
//  - Use Statement: None
//  - Function Name: default
//  - Function Usage: fn run_11() {
//     let d: Vec<u8> = vec![194, 133, 45, 127, 29, 127, 127];
//     let str = str::from_utf8(&d);
//     if let Ok(s) = str {
//         // Creates the cursive root - required for every application.
//         let mut siv = cursive::default();
//
//         // Creates a dialog with a single "Quit" button
//         siv.add_layer(
//             Dialog::around(TextView::new(s))
//                 .title("Cursive")
//                 .button("Quit", |s| s.quit())
//         );
//
//         siv.run();
//     } else {
//         println!("not valid utf8");
//     }
// }
//  - Parameters: initial function signature:{
//   type_name: fn() -> CursiveRunnable,
//   type_fields: [cursive::CursiveRunnable]
// }
// Struct construction metadata: {
//   type_name: fn() -> CursiveRunnable,
//   type_fields: [cursive::CursiveRunnable]
// }
// Struct construction metadata: {
//   type_name: cursive::CursiveRunnable,
//   type_fields: [cursive_core::Cursive, alloc::Box]
// }
// Struct construction metadata: {
//   type_name: cursive_core::Cursive,
//   type_fields: [cursive_core::Theme, cursive_core::OnEventView, cursive_core::Menubar, bool, bool, crossbeam_channel::Receiver, crossbeam_channel::Sender, cursive_core::XY, alloc::Box, core::Option, alloc::Vec]
// }
// Struct construction metadata: {
//   type_name: cursive_core::Theme,
//   type_fields: [bool, cursive_core::BorderStyle, cursive_core::Palette]
// }
// Struct construction metadata: {
//   type_name: cursive_core::BorderStyle,
//   type_fields: [Simple, Outset, None]
// }
// Struct construction metadata: {
//   type_name: cursive_core::Palette,
//   type_fields: [enum_map::EnumMap, std::HashMap, enum_map::EnumMap]
// }
// Struct construction metadata: {
//   type_name: enum_map::EnumMap,
//   type_fields: [cursive_core::PaletteColor, cursive_core::Color]
// }
// Struct construction metadata: {
//   type_name: cursive_core::PaletteColor,
//   type_fields: [Background, Shadow, View, Primary, Secondary, Tertiary, TitlePrimary, TitleSecondary, Highlight, HighlightInactive, HighlightText]
// }
// Struct construction metadata: {
//   type_name: cursive_core::Color,
//   type_fields: [TerminalDefault, Dark(BaseColor), Light(BaseColor), Rgb(u8, u8, u8), RgbLowRes(u8, u8, u8)]
// }
// Struct construction metadata: {
//   type_name: std::HashMap,
//   type_fields: [alloc::String, cursive_core::PaletteNode, ahash::RandomState]
// }
// Struct construction metadata: {
//   type_name: alloc::String,
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
//   type_fields: [*const str]
// }
// Struct construction metadata: {
//   type_name: core::UsizeNoHighBit,
//   type_fields: [usize]
// }
// Struct construction metadata: {
//   type_name: cursive_core::PaletteNode,
//   type_fields: [Color(Color), Namespace(HashMap<String, PaletteNode>)]
// }
// Struct construction metadata: {
//   type_name: ahash::RandomState,
//   type_fields: [u64, u64, u64, u64]
// }
// Struct construction metadata: {
//   type_name: cursive_core::OnEventView,
//   type_fields: [cursive_core::ScreensView, alloc::Vec]
// }
// Struct construction metadata: {
//   type_name: cursive_core::ScreensView,
//   type_fields: [alloc::Vec, usize]
// }
// Struct construction metadata: {
//   type_name: cursive_core::Menubar,
//   type_fields: [cursive_core::Tree, bool, usize, cursive_core::State]
// }
// Struct construction metadata: {
//   type_name: cursive_core::Tree,
//   type_fields: [alloc::Vec]
// }
// Struct construction metadata: {
//   type_name: cursive_core::State,
//   type_fields: [Inactive, Selected, Submenu]
// }
// Struct construction metadata: {
//   type_name: crossbeam_channel::Receiver,
//   type_fields: [cursive_core::Cursive, alloc::Global]
// }
// Struct construction metadata: {
//   type_name: crossbeam_channel::Sender,
//   type_fields: [cursive_core::Cursive, alloc::Global]
// }
// Struct construction metadata: {
//   type_name: cursive_core::XY,
//   type_fields: [usize, usize]
// }
// Struct construction metadata: {
//   type_name: alloc::Box,
//   type_fields: [core::Unique, alloc::Global]
// }
// Struct construction metadata: {
//   type_name: core::Option,
//   type_fields: [None, Some(T)]
// }
