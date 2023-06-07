use super::{Format, Packable};
use core::iter;

impl Packable for f32 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        buf.extend(iter::once(Format::FLOAT32).chain(self.to_be_bytes()));
        5
    }
}

impl Packable for f64 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        buf.extend(iter::once(Format::FLOAT64).chain(self.to_be_bytes()));
        9
    }
}
