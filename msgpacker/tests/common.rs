use core::fmt;
use msgpacker::prelude::*;
use proptest::prelude::*;

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

proptest! {
    #[test]
    fn array(a: [i32; 4]) {
        let mut bytes = vec![];
        let c = a.pack(&mut bytes);
        let (b, x) = <[i32; 4]>::unpack(&bytes).unwrap();
        assert_eq!(c, b);
        assert_eq!(a, x);
    }

    #[test]
    fn tuple(a: (i32, String, bool, usize)) {
        let mut bytes = vec![];
        let c = a.pack(&mut bytes);
        let (b, x) = <(i32, String, bool, usize)>::unpack(&bytes).unwrap();
        assert_eq!(c, b);
        assert_eq!(a, x);
    }
}
