use proptest::prelude::*;

mod utils;

proptest! {
    #[test]
    fn u8(x: u8) {
        utils::case(x);
    }

    #[test]
    fn u16(x: u16) {
        utils::case(x);
    }

    #[test]
    fn u32(x: u32) {
        utils::case(x);
    }

    #[test]
    fn u64(x: u64) {
        utils::case(x);
    }

    #[test]
    fn u128(x: u128) {
        utils::case(x);
    }

    #[test]
    fn usize(x: usize) {
        utils::case(x);
    }

    #[test]
    fn i8(x: i8) {
        utils::case(x);
    }

    #[test]
    fn i16(x: i16) {
        utils::case(x);
    }

    #[test]
    fn i32(x: i32) {
        utils::case(x);
    }

    #[test]
    fn i64(x: i64) {
        utils::case(x);
    }

    #[test]
    fn i128(x: i128) {
        utils::case(x);
    }

    #[test]
    fn isize(x: isize) {
        utils::case(x);
    }
}
