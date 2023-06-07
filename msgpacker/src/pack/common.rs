use super::{Format, Packable};
use core::{iter, marker::PhantomData};

impl Packable for () {
    fn pack<T>(&self, _buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        0
    }
}

impl<X> Packable for PhantomData<X> {
    fn pack<T>(&self, _buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        0
    }
}

impl Packable for bool {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self {
            buf.extend(iter::once(Format::TRUE));
        } else {
            buf.extend(iter::once(Format::FALSE));
        }
        1
    }
}

impl<X> Packable for Option<X>
where
    X: Packable,
{
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Some(t) => t.pack(buf),
            None => {
                buf.extend(iter::once(Format::NIL));
                1
            }
        }
    }
}
