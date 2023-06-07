use msgpacker::prelude::*;
use proptest::prelude::*;

#[test]
fn empty_vec() {
    let v = vec![];
    let mut bytes = vec![];
    v.pack(&mut bytes);
    let (_, x) = Vec::<u8>::unpack(&bytes).unwrap();
    assert_eq!(v, x);
}

#[test]
fn empty_str() {
    let s = "";
    let mut bytes = vec![];
    s.pack(&mut bytes);
    let (_, x) = String::unpack(&bytes).unwrap();
    assert_eq!(s, x);
}

proptest! {
    #[test]
    fn vec(v: Vec<u8>) {
        let mut bytes = Vec::with_capacity(v.len() + 16);
        v.pack(&mut bytes);
        let (_, x) = Vec::<u8>::unpack(&bytes).unwrap();
        assert_eq!(v, x);
    }

    #[test]
    fn str(s: String) {
        let mut bytes = Vec::with_capacity(s.len() + 16);
        s.pack(&mut bytes);
        let (_, x) = String::unpack(&bytes).unwrap();
        assert_eq!(s, x);
    }

    #[test]
    #[ignore]
    fn large_vec(v in prop::collection::vec(any::<u8>(), 0..=u16::MAX as usize * 2)) {
        let mut bytes = Vec::with_capacity(v.len() + 16);
        v.pack(&mut bytes);
        let (_, x) = Vec::<u8>::unpack(&bytes).unwrap();
        assert_eq!(v, x);
    }

    #[test]
    #[ignore]
    fn large_str(v in prop::collection::vec(any::<char>(), 0..=u16::MAX as usize * 2)) {
        let s: String = v.iter().collect();
        let mut bytes = Vec::with_capacity(s.len() + 16);
        s.pack(&mut bytes);
        let (_, x) = String::unpack(&bytes).unwrap();
        assert_eq!(s, x);
    }
}
