use msgpacker::prelude::*;
use proptest::prelude::*;

mod utils;

#[test]
fn nil() {
    utils::case(());
}

#[test]
fn bool() {
    utils::case(true);
    utils::case(false);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, MsgPacker, proptest_derive::Arbitrary)]
pub enum Foo {
    Bar,
    Baz(u32, String),
    Qux { a: Vec<u8>, b: u64 },
}

proptest! {
    #[test]
    fn array(a: [i32; 4]) {
        utils::case(a);
    }

    #[test]
    fn tuple(a: (i32, String, bool, usize)) {
        utils::case(a);
    }

    #[test]
    fn enum_foo(a: Foo) {
        utils::case(a);
    }
}
