use msgpacker::prelude::*;

#[allow(unused)]
pub fn case<T>(x: T)
where
    T: Packable + Unpackable + PartialEq + core::fmt::Debug,
    <T as Unpackable>::Error: core::fmt::Debug,
{
    let mut bytes = vec![];
    let n = x.pack(&mut bytes);
    assert_eq!(n, bytes.len());
    let (o, y) = T::unpack(&bytes).unwrap();
    let (p, z) = T::unpack_iter(bytes).unwrap();
    assert_eq!(n, o);
    assert_eq!(n, p);
    assert_eq!(x, y);
    assert_eq!(x, z);
}
