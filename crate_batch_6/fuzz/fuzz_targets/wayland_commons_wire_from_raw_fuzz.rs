#![no_main]
use libfuzzer_sys::fuzz_target;
use wayland_commons::wire::{Message, ArgumentType as WaylandArg};
use arbitrary::Arbitrary;
use std::os::unix::io::RawFd;

#[derive(Arbitrary, Clone, Debug)]
#[repr(u8)]
pub enum FuzzArgType {
    Int,
    Uint,
    Fixed,
    Str,
    Object,
    NewId,
    Array,
    Fd,
}

impl From<FuzzArgType> for WaylandArg {
    fn from(f: FuzzArgType) -> Self {
        match f {
            FuzzArgType::Int => WaylandArg::Int,
            FuzzArgType::Uint => WaylandArg::Uint,
            FuzzArgType::Fixed => WaylandArg::Fixed,
            FuzzArgType::Str => WaylandArg::Str,
            FuzzArgType::Object => WaylandArg::Object,
            FuzzArgType::NewId => WaylandArg::NewId,
            FuzzArgType::Array => WaylandArg::Array,
            FuzzArgType::Fd => WaylandArg::Fd,
        }
    }
}

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    data: Vec<u32>,
    args: Vec<FuzzArgType>,
    fds: Vec<RawFd>,
}

fuzz_target!(|input: FuzzInput| {
    // Ensure we have some data to work with
    if input.data.is_empty() || input.args.is_empty() || input.fds.is_empty() {
        return;
    }

    // Convert FuzzArgType to WaylandArg
    let converted_args: Vec<WaylandArg> = input.args.iter().cloned().map(|x| x.into()).collect();

    // Catch any panics during parsing
    let result = std::panic::catch_unwind(|| {
        Message::from_raw(&input.data, &converted_args, &input.fds)
    });

    if result.is_err() {
        eprintln!("❌ Caught panic in `Message::from_raw`");
    }
});
