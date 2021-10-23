use std::{io, mem};

pub fn take<R>(reader: &mut R, buf: &mut [u8], len: usize) -> io::Result<()>
where
    R: io::Read,
{
    reader.read_exact(&mut buf[..len])?;

    Ok(())
}

/// Read a slice from the buffered reader.
///
/// # Safety
///
/// Assume the underlying bytes of `R` will live as long as `'a` lives. Otherwise, might cause
/// undefined behavior.
pub unsafe fn take_buf<'a, 'r, R>(reader: &'r mut R, len: usize) -> io::Result<&'a [u8]>
where
    R: io::BufRead,
{
    let r = reader as *mut R;
    let buf = reader.fill_buf()?;

    // Safety: the security assumption of this function is that `'a` won't be dropped while the ref
    // lives
    let buf: &'static [u8] = mem::transmute(buf);

    // Safety: the reader should not drop the data after the consume
    { &mut *r }.consume(len);

    if buf.len() < len {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "The buffer provided to parse the message as reference is not big enough!",
        ));
    }

    Ok(&buf[..len])
}

/// Add a conversion from arbitrary slices into arrays
pub fn from_slice_unchecked<const N: usize>(buf: &[u8]) -> [u8; N] {
    let ptr = buf.as_ptr() as *const [u8; N];

    // Static assertions are not applicable to runtime length check (e.g. slices).
    // This is safe if the size of `bytes` is consistent to `N`
    unsafe { *ptr }
}

pub fn put<W>(writer: &mut W, buf: &[u8]) -> io::Result<usize>
where
    W: io::Write,
{
    writer.write(buf)
}
