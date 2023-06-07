use super::{Format, Packable};
use core::{borrow::Borrow, iter};

/// Packs an array into the extendable buffer, returning the amount of written bytes.
#[allow(unreachable_code)]
pub fn pack_array<T, A, I, V>(buf: &mut T, iter: A) -> usize
where
    T: Extend<u8>,
    A: IntoIterator<IntoIter = I>,
    I: Iterator<Item = V> + ExactSizeIterator,
    V: Packable,
{
    let values = iter.into_iter();
    let len = values.len();
    let n = if len <= 15 {
        buf.extend(iter::once(((len & 0x0f) as u8) | 0x90));
        1
    } else if len <= u16::MAX as usize {
        buf.extend(iter::once(Format::ARRAY16).chain((len as u16).to_be_bytes()));
        3
    } else if len <= u32::MAX as usize {
        buf.extend(iter::once(Format::ARRAY32).chain((len as u32).to_be_bytes()));
        5
    } else {
        #[cfg(feature = "strict")]
        panic!("strict serialization enabled; the buffer is too large");
        return 0;
    };
    n + values.map(|v| v.pack(buf)).sum::<usize>()
}

/// Packs a map into the extendable buffer, returning the amount of written bytes.
#[allow(unreachable_code)]
pub fn pack_map<T, A, I, B, K, V>(buf: &mut T, iter: A) -> usize
where
    T: Extend<u8>,
    A: IntoIterator<IntoIter = I>,
    B: Borrow<(K, V)>,
    I: Iterator<Item = B> + ExactSizeIterator,
    K: Packable,
    V: Packable,
{
    let map = iter.into_iter();
    let len = map.len();
    let n = if len <= 15 {
        buf.extend(iter::once(((len & 0x0f) as u8) | 0x80));
        1
    } else if len <= u16::MAX as usize {
        buf.extend(iter::once(Format::MAP16).chain((len as u16).to_be_bytes()));
        3
    } else if len <= u32::MAX as usize {
        buf.extend(iter::once(Format::MAP32).chain((len as u32).to_be_bytes()));
        5
    } else {
        #[cfg(feature = "strict")]
        panic!("strict serialization enabled; the buffer is too large");
        return 0;
    };
    n + map
        .map(|b| {
            let (k, v) = b.borrow();
            k.pack(buf) + v.pack(buf)
        })
        .sum::<usize>()
}
