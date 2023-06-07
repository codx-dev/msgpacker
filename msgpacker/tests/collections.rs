use core::marker::PhantomData;
use msgpacker::prelude::*;
use proptest::prelude::*;
use std::collections::HashMap;

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    MsgPacker,
    proptest_derive::Arbitrary,
)]
struct Value {
    pub t00: Option<u8>,
    pub t01: Option<u16>,
    pub t02: Option<u32>,
    pub t03: Option<u64>,
    pub t04: Option<usize>,
    pub t05: Option<i8>,
    pub t06: Option<i16>,
    pub t07: Option<i32>,
    pub t08: Option<i64>,
    pub t09: Option<isize>,
    pub t10: (),
    pub t11: PhantomData<String>,
    pub t12: Option<bool>,
    pub t13: Option<Vec<u8>>,
    pub t14: Option<String>,
}

proptest! {
    #[test]
    fn array(value: Vec<Value>) {
        let mut bytes = Vec::new();
        msgpacker::pack_array(&mut bytes, &value);
        let (_, x): (usize, Vec<Value>) = msgpacker::unpack_array(&bytes).unwrap();
        let (_, y): (usize, Vec<Value>) = msgpacker::unpack_array_iter(bytes).unwrap();
        assert_eq!(value, x);
        assert_eq!(value, y);
    }

    #[test]
    fn map(map: HashMap<Value, Value>) {
        let mut bytes = Vec::new();
        msgpacker::pack_map(&mut bytes, &map);
        let (_, x): (usize, HashMap<Value, Value>) = msgpacker::unpack_map(&bytes).unwrap();
        let (_, y): (usize, HashMap<Value, Value>) = msgpacker::unpack_map_iter(bytes).unwrap();
        assert_eq!(map, x);
        assert_eq!(map, y);
    }
}
