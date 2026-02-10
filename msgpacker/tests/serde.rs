use core::marker::PhantomData;

use arbitrary::{Arbitrary as _, Unstructured};
use arbitrary_json::ArbitraryValue;
use msgpacker::Packable;
use msgpacker_derive::MsgPacker;
use proptest::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, MsgPacker)]
pub struct Bar {
    #[serde(with = "serde_bytes")]
    pub b: Vec<u8>,
    pub s: String,
    pub t: (u64, u64, bool, String),
    pub u: (),
    pub p: PhantomData<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, MsgPacker)]
pub enum Foo {
    Bar,
    Baz(u32, String),
    Qux {
        #[serde(with = "serde_bytes")]
        a: Vec<u8>,
        b: u64,
    },
}

#[test]
fn serde_works_bool() {
    case(true);
}

#[test]
fn serde_works_i8() {
    case(i8::MAX - 3);
}

#[test]
fn serde_works_i16() {
    case(i16::MAX - 3);
}

#[test]
fn serde_works_i32() {
    case(i32::MAX - 3);
}

#[test]
fn serde_works_i64() {
    case(i64::MAX - 3);
}

#[test]
fn serde_works_i128() {
    case(i128::MAX - 3);
}

#[test]
fn serde_works_isize() {
    case(isize::MAX - 3);
}

#[test]
fn serde_works_u8() {
    case(u8::MAX - 3);
}

#[test]
fn serde_works_u16() {
    case(u16::MAX - 3);
}

#[test]
fn serde_works_u32() {
    case(u32::MAX - 3);
}

#[test]
fn serde_works_u64() {
    case(u64::MAX - 3);
}

#[test]
fn serde_works_u128() {
    case(u128::MAX - 3);
}

#[test]
fn serde_works_usize() {
    case(usize::MAX - 3);
}

#[test]
fn serde_works_f32() {
    case(f32::MAX - 3.0);
}

#[test]
fn serde_works_f64() {
    case(f64::MAX - 3.0);
}

#[test]
fn serde_works_char() {
    case('x');
}

#[test]
fn serde_works_string() {
    case("foo".to_string());
}

#[test]
fn serde_works_bytes() {
    // we need to specify to serialize this vec as bytes since serde cannot distinguish the
    // concrete type of vec, but messagepack has a special treatment for bytes array
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, MsgPacker)]
    struct BytesWrapper(#[serde(with = "serde_bytes")] Vec<u8>);
    case(BytesWrapper(b"foo".to_vec()));
}

#[test]
fn serde_works_option_some() {
    case(Some("foo".to_string()));
}

#[test]
fn serde_works_option_none() {
    case(Option::<f64>::None);
}

#[test]
fn serde_works_unit() {
    case(());
}

#[test]
fn serde_works_tuple() {
    case((18, u64::MAX, false, "zz".to_string()));
}

#[test]
fn serde_works_struct() {
    case(Bar {
        b: b"xxxx".to_vec(),
        s: "yyy".to_string(),
        t: (18, u64::MAX, false, "zz".to_string()),
        u: (),
        p: PhantomData,
    });
}

#[test]
fn serde_works_enum_variant() {
    case(Foo::Bar);
}

#[test]
fn serde_works_enum_variant_tuple() {
    case(Foo::Baz(15, "xxx".into()));
}

#[test]
fn serde_works_enum_struct() {
    case(Foo::Qux {
        a: vec![1, 2, 3],
        b: 42,
    });
}

#[test]
fn serde_non_uniform_deserialization_to_json() {
    let bytes = vec![146u8, 0, 203, 66, 120, 167, 66, 234, 244, 144, 0];
    let value: Value = msgpacker::serde::from_slice(&bytes).unwrap();
    let value = value.as_array().cloned().unwrap();

    assert_eq!(value[0].as_number().unwrap(), &Number::from(0u64));
    assert_eq!(
        value[1].as_number().unwrap(),
        &Number::from_f64(1694166331209.0).unwrap()
    );
}

proptest! {
    #[test]
    fn serde_proptest_json(seed: [u8; 32]) {
        let seed = Unstructured::new(&seed);
        let value = ArbitraryValue::arbitrary_take_rest(seed).unwrap().take();

        let mut bytes = vec![];
        msgpacker::serde::to_buffer(&mut bytes, &value);

        let y: Value = msgpacker::serde::from_slice(&bytes).unwrap();
        assert_eq!(value, y);
    }
}

pub fn case<T>(x: T)
where
    T: Packable + Serialize + for<'de> Deserialize<'de> + PartialEq + core::fmt::Debug,
{
    let mut bytes = vec![];

    msgpacker::serde::to_buffer(&mut bytes, &x);

    let mut mbt = vec![];
    x.pack(&mut mbt);
    assert_eq!(bytes, mbt);

    let y: T = msgpacker::serde::from_slice(&bytes).unwrap();

    assert_eq!(x, y);
}
