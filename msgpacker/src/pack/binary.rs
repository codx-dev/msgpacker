use super::{Format, Packable};
use core::iter;

impl Packable for [u8] {
    #[allow(unreachable_code)]
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        let n = if self.len() <= u8::MAX as usize {
            buf.extend(iter::once(Format::BIN8).chain(iter::once(self.len() as u8)));
            2
        } else if self.len() <= u16::MAX as usize {
            buf.extend(iter::once(Format::BIN16).chain((self.len() as u16).to_be_bytes()));
            3
        } else if self.len() <= u32::MAX as usize {
            buf.extend(iter::once(Format::BIN32).chain((self.len() as u32).to_be_bytes()));
            5
        } else {
            #[cfg(feature = "strict")]
            panic!("strict serialization enabled; the buffer is too large");
            return 0;
        };
        buf.extend(self.iter().copied());
        n + self.len()
    }
}

#[allow(unreachable_code)]
impl Packable for str {
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        let n = if self.len() <= 31 {
            buf.extend(iter::once((self.len() as u8 & 0x1f) | 0xa0));
            1
        } else if self.len() <= u8::MAX as usize {
            buf.extend(iter::once(Format::STR8).chain(iter::once(self.len() as u8)));
            2
        } else if self.len() <= u16::MAX as usize {
            buf.extend(iter::once(Format::STR16).chain((self.len() as u16).to_be_bytes()));
            3
        } else if self.len() <= u32::MAX as usize {
            buf.extend(iter::once(Format::STR32).chain((self.len() as u32).to_be_bytes()));
            5
        } else {
            #[cfg(feature = "strict")]
            panic!("strict serialization enabled; the buffer is too large");
            return 0;
        };
        buf.extend(self.as_bytes().iter().copied());
        n + self.len()
    }
}

#[cfg(feature = "alloc")]
mod alloc {
    use super::*;
    use ::alloc::{string::String, vec::Vec};

    impl Packable for Vec<u8> {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            self.as_slice().pack(buf)
        }
    }

    impl Packable for String {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            self.as_str().pack(buf)
        }
    }
}
