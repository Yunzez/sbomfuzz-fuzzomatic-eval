#![no_main]
use libfuzzer_sys::fuzz_target;
use wayland_commons::wire::{Message, ArgumentType as WaylandArg};
use arbitrary::{ Arbitrary, Unstructured };
use std::os::raw::c_int;
use std::os::unix::io::RawFd;
use std::{mem, slice};
#[derive(Arbitrary, Clone, Debug)]
#[repr(u32)]
pub enum FuzzArgType {
    Fixed,
    Uint,
    Array,
    Fd,
    NewId,
    Int,
    Str,
    Object,
}

impl From<FuzzArgType> for WaylandArg {
    fn from(f: FuzzArgType) -> Self {
        match f {
            FuzzArgType::Fixed => WaylandArg::Fixed,
            FuzzArgType::Uint => WaylandArg::Uint,
            FuzzArgType::Array => WaylandArg::Array,
            FuzzArgType::Fd => WaylandArg::Fd,
            FuzzArgType::NewId => WaylandArg::NewId,
            FuzzArgType::Int => WaylandArg::Int,
            FuzzArgType::Str => WaylandArg::Str,
            FuzzArgType::Object => WaylandArg::Object,
        }
    }
}

#[derive(Arbitrary, Debug)]
struct MessageFuzzData {
    raw_data: Vec<u32>,
    arg_types: Vec<FuzzArgType>,
    fds: Vec<c_int>,
}

unsafe fn convert_slice<T: Sized>(data: &[u8]) -> &[T] {
    let n = mem::size_of::<T>();
    slice::from_raw_parts(
        data.as_ptr() as *const T,
        data.len()/n,
    )
}

fn get_arg_types(data: &[u8]) -> [WaylandArg; 16] {
    use WaylandArg::*;
    let mut res = [Int; 16];
    assert_eq!(data.len(), 16);
    for i in 0..16 {
        res[i] = match data[i] & 0b111 {
            0 => Int,
            1 => Uint,
            2 => Fixed,
            3 => Str,
            4 => Object,
            5 => NewId,
            6 => Array,
            7 => Fd,
            _ => unreachable!(),
        }
    }
    res
}

fuzz_target!(|data: &[u8]| {
    // let mut unstructured = Unstructured::new(data);

    // if let Ok(MessageFuzzData { raw_data, arg_types, fds }) = MessageFuzzData::arbitrary(&mut unstructured) {
    //     let converted: Vec<WaylandArg> = arg_types.into_iter().map(Into::into).collect();
    //     let _ = std::panic::catch_unwind(|| Message::from_raw(&raw_data, &converted, &fds));
    // }

    if data.len() < 32 { return; }
    // 4 `RawFd`s
    let fds: &[RawFd] = unsafe { convert_slice(&data[..16]) };
    // 16 `ArgumentType`s
    let args = get_arg_types(&data[16..32]);
    let data: &[u32] = unsafe { convert_slice(&data[32..]) };
    let _res = Message::from_raw(data, &args, fds);
});
