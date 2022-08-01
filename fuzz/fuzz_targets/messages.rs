#![no_main]

use std::io;

use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;
use msgpacker::prelude::*;

#[derive(Arbitrary, MsgPacker, Debug, Clone, PartialEq, Eq)]
pub struct AllMessages {
    pub uint64: Option<u64>,
    pub int64: Option<i64>,
    pub b: Option<bool>,
    pub f_32: Option<f32>,
    pub f_64: Option<f64>,
    pub string: Option<String>,
    pub bin: Option<Vec<u8>>,
}

fuzz_target!(|m: AllMessages| {
    let mut m = m;

    let buf: Vec<u8> = vec![];
    let mut packer = io::Cursor::new(buf);

    packer.pack(m.clone()).expect("failed to pack message");
    packer.set_position(0);

    let mut p: AllMessages = packer.unpack().expect("failed to unpack message");

    // NaN equality doesn't hold
    if m.f_32.filter(|f| f.is_nan()).is_some() {
        m.f_32.take();
    }
    if m.f_64.filter(|f| f.is_nan()).is_some() {
        m.f_64.take();
    }
    if p.f_32.filter(|f| f.is_nan()).is_some() {
        p.f_32.take();
    }
    if p.f_64.filter(|f| f.is_nan()).is_some() {
        p.f_64.take();
    }

    assert_eq!(m, p);
});
