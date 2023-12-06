use super::{
    helpers::{take_byte, take_num},
    Error, Format, Unpackable,
};
use core::str;

pub fn unpack_bytes(mut buf: &[u8]) -> Result<(usize, &[u8]), Error> {
    let format = take_byte(&mut buf)?;
    let (n, len) = match format {
        Format::BIN8 => (2, take_byte(&mut buf)? as usize),
        Format::BIN16 => (3, take_num(&mut buf, u16::from_be_bytes)? as usize),
        Format::BIN32 => (5, take_num(&mut buf, u32::from_be_bytes)? as usize),
        _ => return Err(Error::UnexpectedFormatTag),
    };
    if buf.len() < len {
        return Err(Error::BufferTooShort);
    }
    Ok((n + len, &buf[..len]))
}

pub fn unpack_str(mut buf: &[u8]) -> Result<(usize, &str), Error> {
    let format = take_byte(&mut buf)?;
    let (n, len) = match format {
        0xa0..=0xbf => (1, format as usize & 0x1f),
        Format::STR8 => (2, take_byte(&mut buf)? as usize),
        Format::STR16 => (3, take_num(&mut buf, u16::from_be_bytes)? as usize),
        Format::STR32 => (5, take_num(&mut buf, u32::from_be_bytes)? as usize),
        _ => return Err(Error::UnexpectedFormatTag),
    };
    if buf.len() < len {
        return Err(Error::BufferTooShort);
    }
    let str = str::from_utf8(&buf[..len]).map_err(|_| Error::InvalidUtf8)?;
    Ok((n + len, str))
}

#[cfg(feature = "alloc")]
mod alloc {
    use super::*;
    use crate::binary::alloc::MsgPackerBin;
    use crate::helpers::{take_byte_iter, take_num_iter};
    use ::alloc::{string::String, vec::Vec};

    impl Unpackable for MsgPackerBin {
        type Error = Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_bytes(buf).map(|(n, b)| (n, MsgPackerBin(b.to_vec())))
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            let mut bytes = bytes.into_iter();
            let format = take_byte_iter(bytes.by_ref())?;
            let (n, len) = match format {
                Format::BIN8 => (2, take_byte_iter(bytes.by_ref())? as usize),
                Format::BIN16 => (
                    3,
                    take_num_iter(bytes.by_ref(), u16::from_be_bytes)? as usize,
                ),
                Format::BIN32 => (
                    5,
                    take_num_iter(bytes.by_ref(), u32::from_be_bytes)? as usize,
                ),
                _ => return Err(Error::UnexpectedFormatTag),
            };
            let v: Vec<_> = bytes.take(len).collect();
            if v.len() < len {
                return Err(Error::BufferTooShort);
            }
            Ok((n + len, MsgPackerBin(v)))
        }
    }

    impl Unpackable for String {
        type Error = Error;

        fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error> {
            unpack_str(buf).map(|(n, s)| (n, s.into()))
        }

        fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
        where
            I: IntoIterator<Item = u8>,
        {
            let mut bytes = bytes.into_iter();
            let format = take_byte_iter(bytes.by_ref())?;
            let (n, len) = match format {
                0xa0..=0xbf => (1, format as usize & 0x1f),
                Format::STR8 => (2, take_byte_iter(bytes.by_ref())? as usize),
                Format::STR16 => (
                    3,
                    take_num_iter(bytes.by_ref(), u16::from_be_bytes)? as usize,
                ),
                Format::STR32 => (
                    5,
                    take_num_iter(bytes.by_ref(), u32::from_be_bytes)? as usize,
                ),
                _ => return Err(Error::UnexpectedFormatTag),
            };
            let v: Vec<_> = bytes.take(len).collect();
            if v.len() < len {
                return Err(Error::BufferTooShort);
            }
            let s = String::from_utf8(v).map_err(|_| Error::InvalidUtf8)?;
            Ok((n + len, s))
        }
    }
}
