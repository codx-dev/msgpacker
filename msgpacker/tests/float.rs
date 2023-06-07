use msgpacker::prelude::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn f32(x: f32) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = f32::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn f64(x: f64) {
        let mut bytes = vec![];
        x.pack(&mut bytes);
        let (_, y) = f64::unpack(&bytes).unwrap();
        assert_eq!(x, y);
    }
}
