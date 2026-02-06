use super::{Format, Packable};
use core::{borrow::Borrow, iter};

/// Packs the length of an array.
pub fn pack_array_len<T>(buf: &mut T, len: usize) -> usize
where
    T: Extend<u8>,
{
    if len <= 15 {
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
    }
}

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
    let n = pack_array_len(buf, len);
    n + values.map(|v| v.pack(buf)).sum::<usize>()
}

/// Packs the length of a map.
pub fn pack_map_len<T>(buf: &mut T, len: usize) -> usize
where
    T: Extend<u8>,
{
    if len <= 15 {
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
    }
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
    let n = pack_map_len(buf, len);
    n + map
        .map(|b| {
            let (k, v) = b.borrow();
            k.pack(buf) + v.pack(buf)
        })
        .sum::<usize>()
}

#[cfg(feature = "alloc")]
mod alloc {
    use super::*;
    use ::alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

    impl<X> Packable for BTreeSet<X>
    where
        X: Packable,
    {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            pack_array(buf, self)
        }
    }

    impl<X> Packable for BinaryHeap<X>
    where
        X: Packable,
    {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            pack_array(buf, self)
        }
    }

    impl<X> Packable for LinkedList<X>
    where
        X: Packable,
    {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            pack_array(buf, self)
        }
    }

    impl<X> Packable for VecDeque<X>
    where
        X: Packable,
    {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            pack_array(buf, self)
        }
    }

    impl<K, V> Packable for BTreeMap<K, V>
    where
        K: Packable,
        V: Packable,
    {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            pack_map(buf, self)
        }
    }
}

#[cfg(feature = "std")]
mod std {
    use super::*;
    use ::std::collections::{HashMap, HashSet};

    impl<X> Packable for HashSet<X>
    where
        X: Packable,
    {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            pack_array(buf, self)
        }
    }

    impl<K, V> Packable for HashMap<K, V>
    where
        K: Packable,
        V: Packable,
    {
        fn pack<T>(&self, buf: &mut T) -> usize
        where
            T: Extend<u8>,
        {
            pack_map(buf, self)
        }
    }
}
