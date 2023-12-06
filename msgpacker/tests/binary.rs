use msgpacker::prelude::*;
use proptest::prelude::*;

mod utils;

#[test]
fn empty_vec() {
    let v = MsgPackerBin(vec![]);
    let mut bytes = vec![];
    let n = v.pack(&mut bytes);
    let (o, x) = MsgPackerBin::unpack(&bytes).unwrap();
    let (p, y) = MsgPackerBin::unpack_iter(bytes).unwrap();
    assert_eq!(o, n);
    assert_eq!(p, n);
    assert_eq!(v, x);
    assert_eq!(v, y);
}

#[test]
fn empty_str() {
    let s = "";
    let mut bytes = vec![];
    let n = s.pack(&mut bytes);
    let (o, x) = String::unpack(&bytes).unwrap();
    let (p, y) = String::unpack_iter(bytes).unwrap();
    assert_eq!(o, n);
    assert_eq!(p, n);
    assert_eq!(s, x);
    assert_eq!(s, y);
}

proptest! {
    #[test]
    fn str(s: String) {
        utils::case(s);
    }

    #[test]
    #[ignore]
    fn large_str(v in prop::collection::vec(any::<char>(), 0..=u16::MAX as usize * 2)) {
        utils::case(v.into_iter().collect::<String>());
    }
}
