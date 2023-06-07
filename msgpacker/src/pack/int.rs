use super::{Format, Packable};
use core::iter;

impl Packable for u8 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(self & Format::POSITIVE_FIXINT));
            1
        } else {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self)));
            2
        }
    }
}

impl Packable for u16 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as u16 {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else {
            buf.extend(iter::once(Format::UINT16).chain(self.to_be_bytes()));
            3
        }
    }
}

impl Packable for u32 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as u32 {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= u16::MAX as u32 {
            buf.extend(
                iter::once(Format::UINT16).chain(self.to_be_bytes().iter().skip(2).copied()),
            );
            3
        } else {
            buf.extend(iter::once(Format::UINT32).chain(self.to_be_bytes()));
            5
        }
    }
}

impl Packable for u64 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as u64 {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= u16::MAX as u64 {
            buf.extend(
                iter::once(Format::UINT16).chain(self.to_be_bytes().iter().skip(6).copied()),
            );
            3
        } else if *self <= u32::MAX as u64 {
            buf.extend(
                iter::once(Format::UINT32).chain(self.to_be_bytes().iter().skip(4).copied()),
            );
            5
        } else {
            buf.extend(iter::once(Format::UINT64).chain(self.to_be_bytes()));
            9
        }
    }
}

impl Packable for usize {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= 127 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= u8::MAX as usize {
            buf.extend(iter::once(Format::UINT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= u16::MAX as usize {
            buf.extend(
                iter::once(Format::UINT16).chain(self.to_be_bytes().iter().skip(6).copied()),
            );
            3
        } else if *self <= u32::MAX as usize {
            buf.extend(
                iter::once(Format::UINT32).chain(self.to_be_bytes().iter().skip(4).copied()),
            );
            5
        } else {
            buf.extend(iter::once(Format::UINT64).chain(self.to_be_bytes()));
            9
        }
    }
}

impl Packable for i8 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once(*self as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32i8) as u8));
            1
        } else {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        }
    }
}

impl Packable for i16 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i8::MIN as i16 {
            buf.extend(iter::once(Format::INT16).chain(self.to_be_bytes()));
            2
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self as i8 | -32i8) as u8));
            1
        } else if *self <= i8::MAX as i16 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else {
            buf.extend(iter::once(Format::INT16).chain(self.to_be_bytes()));
            2
        }
    }
}

impl Packable for i32 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i16::MIN as i32 {
            buf.extend(iter::once(Format::INT32).chain(self.to_be_bytes()));
            5
        } else if *self < i8::MIN as i32 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            2
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32i32) as u8));
            1
        } else if *self <= i8::MAX as i32 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= i16::MAX as i32 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            2
        } else {
            buf.extend(iter::once(Format::INT32).chain(self.to_be_bytes()));
            5
        }
    }
}

impl Packable for i64 {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i32::MIN as i64 {
            buf.extend(iter::once(Format::INT64).chain(self.to_be_bytes()));
            9
        } else if *self < i16::MIN as i64 {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else if *self < i8::MIN as i64 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            2
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32i64) as u8));
            1
        } else if *self <= i8::MAX as i64 {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= i16::MAX as i64 {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            2
        } else if *self <= i32::MAX as i64 {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else {
            buf.extend(iter::once(Format::INT64).chain(self.to_be_bytes()));
            9
        }
    }
}

impl Packable for isize {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        if *self < i32::MIN as isize {
            buf.extend(iter::once(Format::INT64).chain(self.to_be_bytes()));
            9
        } else if *self < i16::MIN as isize {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else if *self < i8::MIN as isize {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            2
        } else if *self <= -33 {
            buf.extend(iter::once(Format::INT8).chain(iter::once((*self as i8) as u8)));
            2
        } else if *self <= -1 {
            buf.extend(iter::once((*self | -32isize) as u8));
            1
        } else if *self <= i8::MAX as isize {
            buf.extend(iter::once(*self as u8 & Format::POSITIVE_FIXINT));
            1
        } else if *self <= i16::MAX as isize {
            buf.extend(iter::once(Format::INT16).chain((*self as i16).to_be_bytes()));
            2
        } else if *self <= i32::MAX as isize {
            buf.extend(iter::once(Format::INT32).chain((*self as i32).to_be_bytes()));
            5
        } else {
            buf.extend(iter::once(Format::INT64).chain(self.to_be_bytes()));
            9
        }
    }
}
