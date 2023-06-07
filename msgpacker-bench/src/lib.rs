use core::marker::PhantomData;
use rand::{
    distributions::{Alphanumeric, Standard},
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    msgpacker::MsgPacker,
    Serialize,
    Deserialize,
)]
pub struct Value {
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

impl Distribution<Value> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Value {
        let max = u16::MAX as usize + 1024;
        let len_bin = rng.gen_range(0..max);
        let len_txt = rng.gen_range(0..max);
        Value {
            t00: rng.gen(),
            t01: rng.gen(),
            t02: rng.gen(),
            t03: rng.gen(),
            t04: rng.gen(),
            t05: rng.gen(),
            t06: rng.gen(),
            t07: rng.gen(),
            t08: rng.gen(),
            t09: rng.gen(),
            t10: (),
            t11: PhantomData,
            t12: rng.gen(),
            t13: rng
                .gen::<bool>()
                .then(|| rng.sample_iter(&Standard).take(len_bin).collect()),
            t14: rng.gen::<bool>().then(|| {
                rng.sample_iter(&Alphanumeric)
                    .map(char::from)
                    .take(len_txt)
                    .collect()
            }),
        }
    }
}
