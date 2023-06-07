use core::fmt;
use msgpacker::prelude::*;

#[test]
fn nil() {
    pack_unpack((), 0);
}

#[test]
fn bool() {
    pack_unpack(true, 1);
    pack_unpack(false, 1);
}

fn pack_unpack<T>(t: T, len: usize)
where
    T: Packable + Unpackable + PartialEq + fmt::Debug,
    <T as Unpackable>::Error: fmt::Debug,
{
    let mut bytes = vec![];
    let a = t.pack(&mut bytes);
    let (b, x) = T::unpack(&bytes).unwrap();
    assert_eq!(a, len);
    assert_eq!(b, len);
    assert_eq!(t, x);
}
