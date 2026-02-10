use msgpacker::prelude::*;
use proptest::prelude::*;

mod utils;

#[test]
fn vec_u8_uses_binary_format() {
    #[derive(Debug, PartialEq, Eq, MsgPacker)]
    struct WithBytes {
        data: Vec<u8>,
    }

    let val = WithBytes {
        data: vec![0xde, 0xad, 0xbe, 0xef],
    };

    let mut bytes = vec![];
    let n = val.pack(&mut bytes);
    assert_eq!(n, bytes.len());

    // The struct has a single Vec<u8> field, so the serialized output should
    // start with a msgpack binary format tag (BIN8=0xc4, BIN16=0xc5, BIN32=0xc6),
    // NOT an array format tag (0x90..=0x9f / 0xdc / 0xdd).
    let tag = bytes[0];
    assert!(
        matches!(tag, 0xc4 | 0xc5 | 0xc6,),
        "expected binary format tag (0xc4..=0xc6), got 0x{:02x}",
        tag,
    );

    // For 4 bytes the format should be BIN8 (0xc4), length 4, then the payload.
    assert_eq!(bytes[0], 0xc4); // BIN8
    assert_eq!(bytes[1], 4); // length
    assert_eq!(&bytes[2..6], &[0xde, 0xad, 0xbe, 0xef]);

    // Round-trip via slice and iterator.
    let (o, deserialized) = WithBytes::unpack(&bytes).unwrap();
    assert_eq!(o, n);
    assert_eq!(val, deserialized);

    let (p, deserialized_iter) = WithBytes::unpack_iter(bytes).unwrap();
    assert_eq!(p, n);
    assert_eq!(val, deserialized_iter);
}

#[test]
fn vec_string_uses_array_format() {
    #[derive(Debug, PartialEq, Eq, MsgPacker)]
    struct WithStrings {
        data: Vec<String>,
    }

    let val = WithStrings {
        data: vec!["hello".into(), "world".into()],
    };

    let mut bytes = vec![];
    let n = val.pack(&mut bytes);
    assert_eq!(n, bytes.len());

    // The struct has a single Vec<String> field, so the serialized output should
    // start with a msgpack array format tag (fixarray=0x90..=0x9f, array16=0xdc,
    // array32=0xdd), NOT a binary format tag.
    let tag = bytes[0];
    assert!(
        matches!(tag, 0x90..=0x9f | 0xdc | 0xdd),
        "expected array format tag, got 0x{:02x}",
        tag,
    );

    // Round-trip via slice and iterator.
    let (o, deserialized) = WithStrings::unpack(&bytes).unwrap();
    assert_eq!(o, n);
    assert_eq!(val, deserialized);

    let (p, deserialized_iter) = WithStrings::unpack_iter(bytes).unwrap();
    assert_eq!(p, n);
    assert_eq!(val, deserialized_iter);
}

#[test]
fn empty_vec() {
    let v = vec![];
    let mut bytes = vec![];
    let n = v.pack(&mut bytes);
    let (o, x) = Vec::<u8>::unpack(&bytes).unwrap();
    let (p, y) = Vec::<u8>::unpack_iter(bytes).unwrap();
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
    fn vec(v: Vec<u8>) {
        utils::case(v);
    }

    #[test]
    fn str(s: String) {
        utils::case(s);
    }

    #[test]
    #[ignore]
    fn large_vec(v in prop::collection::vec(any::<u8>(), 0..=u16::MAX as usize * 2)) {
        utils::case(v);
    }

    #[test]
    #[ignore]
    fn large_str(v in prop::collection::vec(any::<char>(), 0..=u16::MAX as usize * 2)) {
        utils::case(v.into_iter().collect::<String>());
    }
}
