use msgpacker::prelude::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn u8(x: u8) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = u8::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn u16(x: u16) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = u16::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn u32(x: u32) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = u32::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn u64(x: u64) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = u64::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn usize(x: usize) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = usize::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn i8(x: i8) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = i8::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn i16(x: i16) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = i16::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn i32(x: i32) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = i32::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn i64(x: i64) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = i64::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn isize(x: isize) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = isize::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }
}
