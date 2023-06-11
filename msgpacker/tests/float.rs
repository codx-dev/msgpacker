use proptest::prelude::*;

mod utils;

proptest! {
    #[test]
    fn f32(x: f32) {
        utils::case(x);
    }

    #[test]
    fn f64(x: f64) {
        utils::case(x);
    }
}
