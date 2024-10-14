use super::Error;

pub fn take_byte_iter<I>(mut bytes: I) -> Result<u8, Error>
where
    I: Iterator<Item = u8>,
{
    bytes.next().ok_or(Error::BufferTooShort)
}

pub fn take_byte(buf: &mut &[u8]) -> Result<u8, Error> {
    if buf.is_empty() {
        return Err(Error::BufferTooShort);
    }
    let (l, r) = buf.split_at(1);
    *buf = r;
    Ok(l[0])
}

pub fn take_num<V, const N: usize>(buf: &mut &[u8], f: fn([u8; N]) -> V) -> Result<V, Error> {
    if buf.len() < N {
        return Err(Error::BufferTooShort);
    }
    let (l, r) = buf.split_at(N);
    *buf = r;
    // Safety: l.len() == N
    let val = unsafe { <[u8; N]>::try_from(l).unwrap_unchecked() };
    Ok(f(val))
}

#[cfg(feature = "alloc")]
pub fn take_buffer<'a>(buf: &mut &'a [u8], len: usize) -> Result<&'a [u8], Error> {
    if buf.len() < len {
        return Err(Error::BufferTooShort);
    }
    let (l, r) = buf.split_at(len);
    *buf = r;
    Ok(l)
}

pub fn take_num_iter<I, V, const N: usize>(bytes: I, f: fn([u8; N]) -> V) -> Result<V, Error>
where
    I: Iterator<Item = u8>,
{
    bytes
        .array_chunks()
        .next()
        .ok_or(Error::BufferTooShort)
        .map(f)
}

#[cfg(feature = "alloc")]
pub fn take_buffer_iter<I>(bytes: I, len: usize) -> Result<alloc::vec::Vec<u8>, Error>
where
    I: Iterator<Item = u8>,
{
    let v: alloc::vec::Vec<_> = bytes.take(len).collect();
    if v.len() < len {
        return Err(Error::BufferTooShort);
    }
    Ok(v)
}
