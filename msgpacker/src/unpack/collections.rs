use super::{
    helpers::{take_byte, take_byte_iter, take_num, take_num_iter},
    Error, Format, Unpackable,
};

/// Unpacks an array from the buffer, returning a collectable type and the amount of read bytes.
pub fn unpack_array<V, C>(mut buf: &[u8]) -> Result<(usize, C), <V as Unpackable>::Error>
where
    V: Unpackable,
    C: FromIterator<V>,
{
    let format = take_byte(&mut buf)?;
    let (mut n, len) = match format {
        0x90..=0x9f => (1, (format & 0x0f) as usize),
        Format::ARRAY16 => (
            3,
            take_num(&mut buf, u16::from_be_bytes).map(|v| v as usize)?,
        ),
        Format::ARRAY32 => (
            5,
            take_num(&mut buf, u32::from_be_bytes).map(|v| v as usize)?,
        ),
        _ => return Err(Error::UnexpectedFormatTag.into()),
    };
    let array: C = (0..len)
        .map(|_| {
            let (count, v) = V::unpack(buf)?;
            buf = &buf[count..];
            n += count;
            Ok(v)
        })
        .collect::<Result<_, <V as Unpackable>::Error>>()?;
    Ok((n, array))
}

/// Unpacks an array from the iterator, returning a collectable type and the amount of read bytes.
pub fn unpack_array_iter<I, V, C>(iter: I) -> Result<(usize, C), <V as Unpackable>::Error>
where
    I: IntoIterator<Item = u8>,
    V: Unpackable,
    C: FromIterator<V>,
{
    let mut bytes = iter.into_iter();
    let format = take_byte_iter(bytes.by_ref())?;
    let (mut n, len) = match format {
        0x90..=0x9f => (1, (format & 0x0f) as usize),
        Format::ARRAY16 => (
            3,
            take_num_iter(bytes.by_ref(), u16::from_be_bytes).map(|v| v as usize)?,
        ),
        Format::ARRAY32 => (
            5,
            take_num_iter(bytes.by_ref(), u32::from_be_bytes).map(|v| v as usize)?,
        ),
        _ => return Err(Error::UnexpectedFormatTag.into()),
    };
    let array: C = (0..len)
        .map(|_| {
            let (count, v) = V::unpack_iter(bytes.by_ref())?;
            n += count;
            Ok(v)
        })
        .collect::<Result<_, <V as Unpackable>::Error>>()?;
    Ok((n, array))
}

/// Unpacks a map from the buffer, returning a collectable type and the amount of read bytes.
pub fn unpack_map<K, V, C>(mut buf: &[u8]) -> Result<(usize, C), <V as Unpackable>::Error>
where
    K: Unpackable,
    V: Unpackable,
    <V as Unpackable>::Error: From<<K as Unpackable>::Error>,
    C: FromIterator<(K, V)>,
{
    let format = take_byte(&mut buf)?;
    let (mut n, len) = match format {
        0x80..=0x8f => (1, (format & 0x0f) as usize),
        Format::MAP16 => (
            3,
            take_num(&mut buf, u16::from_be_bytes).map(|v| v as usize)?,
        ),
        Format::MAP32 => (
            5,
            take_num(&mut buf, u32::from_be_bytes).map(|v| v as usize)?,
        ),
        _ => return Err(Error::UnexpectedFormatTag.into()),
    };
    let map: C = (0..len)
        .map(|_| {
            let (count, k) = K::unpack(buf)?;
            buf = &buf[count..];
            n += count;
            let (count, v) = V::unpack(buf)?;
            buf = &buf[count..];
            n += count;
            Ok((k, v))
        })
        .collect::<Result<_, <V as Unpackable>::Error>>()?;
    Ok((n, map))
}

/// Unpacks a map from the iterator, returning a collectable type and the amount of read bytes.
pub fn unpack_map_iter<I, K, V, C>(iter: I) -> Result<(usize, C), <V as Unpackable>::Error>
where
    I: IntoIterator<Item = u8>,
    K: Unpackable,
    V: Unpackable,
    <V as Unpackable>::Error: From<<K as Unpackable>::Error>,
    C: FromIterator<(K, V)>,
{
    let mut bytes = iter.into_iter();
    let format = take_byte_iter(bytes.by_ref())?;
    let (mut n, len) = match format {
        0x80..=0x8f => (1, (format & 0x0f) as usize),
        Format::MAP16 => (
            3,
            take_num_iter(bytes.by_ref(), u16::from_be_bytes).map(|v| v as usize)?,
        ),
        Format::MAP32 => (
            5,
            take_num_iter(bytes.by_ref(), u32::from_be_bytes).map(|v| v as usize)?,
        ),
        _ => return Err(Error::UnexpectedFormatTag.into()),
    };
    let map: C = (0..len)
        .map(|_| {
            let (count, k) = K::unpack_iter(bytes.by_ref())?;
            n += count;
            let (count, v) = V::unpack_iter(bytes.by_ref())?;
            n += count;
            Ok((k, v))
        })
        .collect::<Result<_, <V as Unpackable>::Error>>()?;
    Ok((n, map))
}

#[cfg(feature = "alloc")]
mod alloc {
    use super::*;
    use ::alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

    impl<X> Unpackable for BTreeSet<X>
    where
        X: Unpackable + Ord,
    {
        type Error = <X as Unpackable>::Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_array(buf)
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            unpack_array_iter(bytes)
        }
    }

    impl<X> Unpackable for BinaryHeap<X>
    where
        X: Unpackable + Ord,
    {
        type Error = <X as Unpackable>::Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_array(buf)
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            unpack_array_iter(bytes)
        }
    }

    impl<X> Unpackable for LinkedList<X>
    where
        X: Unpackable,
    {
        type Error = <X as Unpackable>::Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_array(buf)
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            unpack_array_iter(bytes)
        }
    }

    impl<X> Unpackable for VecDeque<X>
    where
        X: Unpackable,
    {
        type Error = <X as Unpackable>::Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_array(buf)
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            unpack_array_iter(bytes)
        }
    }

    impl<K, V> Unpackable for BTreeMap<K, V>
    where
        K: Unpackable + Ord,
        V: Unpackable,
        <V as Unpackable>::Error: From<<K as Unpackable>::Error>,
    {
        type Error = <V as Unpackable>::Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_map(buf)
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            unpack_map_iter(bytes)
        }
    }
}

#[cfg(feature = "std")]
mod std {
    use super::*;
    use ::std::{
        collections::{HashMap, HashSet},
        hash::Hash,
    };

    impl<X> Unpackable for HashSet<X>
    where
        X: Unpackable + Hash + Eq,
    {
        type Error = <X as Unpackable>::Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_array(buf)
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            unpack_array_iter(bytes)
        }
    }

    impl<K, V> Unpackable for HashMap<K, V>
    where
        K: Unpackable + Hash + Eq,
        V: Unpackable,
        <V as Unpackable>::Error: From<<K as Unpackable>::Error>,
    {
        type Error = <V as Unpackable>::Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_map(buf)
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            unpack_map_iter(bytes)
        }
    }
}
