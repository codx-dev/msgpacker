use super::{
    helpers::{take_byte, take_byte_iter},
    Error, Format, Unpackable,
};
use core::marker::PhantomData;

impl Unpackable for () {
    type Error = Error;

    fn unpack(_buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        Ok((0, ()))
    }

    fn unpack_iter<I>(_buf: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        Ok((0, ()))
    }
}

impl<X> Unpackable for PhantomData<X> {
    type Error = Error;

    fn unpack(_buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        Ok((0, PhantomData))
    }

    fn unpack_iter<I>(_buf: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        Ok((0, PhantomData))
    }
}

impl Unpackable for bool {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        if format != Format::TRUE && format != Format::FALSE {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((1, format != Format::FALSE))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        if format != Format::TRUE && format != Format::FALSE {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((1, format != Format::FALSE))
    }
}

impl<X> Unpackable for Option<X>
where
    X: Unpackable,
{
    type Error = <X as Unpackable>::Error;

    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        if buf.is_empty() {
            return Err(Error::BufferTooShort.into());
        }
        if buf[0] == Format::NIL {
            return Ok((1, None));
        }
        X::unpack(buf).map(|(n, x)| (n, Some(x)))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter().peekable();
        let format = *bytes.peek().ok_or(Error::BufferTooShort)?;
        if format == Format::NIL {
            bytes.next();
            return Ok((1, None));
        }
        X::unpack_iter(bytes).map(|(n, x)| (n, Some(x)))
    }
}
