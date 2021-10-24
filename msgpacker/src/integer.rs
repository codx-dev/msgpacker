use crate::buffer;
use crate::format::MessageFormat;

use std::io;

/// Integer representation for msgpack
///
/// Only 64-bits integers are available because their memory representation is regardless of their
/// canonical format in the implementation. The protocol definition clearly states the integer
/// should be serialized in the most optimal representation possible considering their current
/// value.
///
/// [specs](https://github.com/msgpack/msgpack/blob/master/spec.md#int-format-family)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Integer {
    /// Unsigned int representation
    Uint64(u64),
    /// Signed int representation
    Int64(i64),
}

impl Integer {
    /// Create a new unsigned interger
    pub fn unsigned<T>(n: T) -> Self
    where
        T: Into<u64>,
    {
        Self::Uint64(n.into())
    }

    /// Create a new signed interger
    pub fn signed<T>(n: T) -> Self
    where
        T: Into<i64>,
    {
        Self::Int64(n.into())
    }

    /// Return either a raw i64 or a cast u64
    pub const fn as_signed(&self) -> i64 {
        match self {
            Self::Uint64(i) => *i as i64,
            Self::Int64(i) => *i,
        }
    }

    /// Return either a raw u64 or a cast i64
    pub const fn as_unsigned(&self) -> u64 {
        match self {
            Self::Uint64(i) => *i,
            Self::Int64(i) => *i as u64,
        }
    }

    /// Return the underlying u64, if the number if unsigned
    pub const fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Uint64(n) => Some(*n),
            _ => None,
        }
    }

    /// Return the underlying i64, if the number if signed
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int64(n) => Some(*n),
            _ => None,
        }
    }

    /// Pack this integer into writer and return the amount of bytes written
    pub fn pack<W>(&self, mut writer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let mut n = 0;

        match self {
            Self::Int64(i) if *i <= i32::MIN as i64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Int64)])?;
                n += buffer::put(&mut writer, &i.to_be_bytes())?;
            }

            Self::Int64(i) if *i <= i16::MIN as i64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Int32)])?;
                n += buffer::put(&mut writer, &(*i as i32).to_be_bytes())?;
            }

            Self::Int64(i) if *i <= i8::MIN as i64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Int16)])?;
                n += buffer::put(&mut writer, &(*i as i16).to_be_bytes())?;
            }

            Self::Int64(i) if *i <= -33 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Int8)])?;
                n += buffer::put(&mut writer, &(*i as i8).to_be_bytes())?;
            }

            Self::Int64(i) if *i <= -1 => {
                n += buffer::put(
                    &mut writer,
                    &[u8::from(MessageFormat::NegativeFixInt(*i as i8))],
                )?
            }

            Self::Int64(i) if *i <= 127 => {
                n += buffer::put(
                    &mut writer,
                    &[u8::from(MessageFormat::PositiveFixint(*i as u8))],
                )?
            }

            Self::Uint64(i) if *i <= 127 => {
                n += buffer::put(
                    &mut writer,
                    &[u8::from(MessageFormat::PositiveFixint(*i as u8))],
                )?
            }

            Self::Int64(i) if *i <= u8::MAX as i64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Uint8)])?;
                n += buffer::put(&mut writer, &(*i as u8).to_be_bytes())?;
            }

            Self::Uint64(i) if *i <= u8::MAX as u64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Uint8)])?;
                n += buffer::put(&mut writer, &(*i as u8).to_be_bytes())?;
            }

            Self::Int64(i) if *i <= u16::MAX as i64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Uint16)])?;
                n += buffer::put(&mut writer, &(*i as u16).to_be_bytes())?;
            }

            Self::Uint64(i) if *i <= u16::MAX as u64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Uint16)])?;
                n += buffer::put(&mut writer, &(*i as u16).to_be_bytes())?;
            }

            Self::Int64(i) if *i <= u32::MAX as i64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Uint32)])?;
                n += buffer::put(&mut writer, &(*i as u32).to_be_bytes())?;
            }

            Self::Uint64(i) if *i <= u32::MAX as u64 => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Uint32)])?;
                n += buffer::put(&mut writer, &(*i as u32).to_be_bytes())?;
            }

            Self::Int64(i) => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Uint64)])?;
                n += buffer::put(&mut writer, &i.to_be_bytes())?;
            }

            Self::Uint64(i) => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Uint64)])?;
                n += buffer::put(&mut writer, &i.to_be_bytes())?;
            }
        }

        Ok(n)
    }
}

impl From<Integer> for i64 {
    fn from(i: Integer) -> i64 {
        match i {
            Integer::Uint64(u) => u as i64,
            Integer::Int64(i) => i,
        }
    }
}

impl From<Integer> for u64 {
    fn from(i: Integer) -> u64 {
        match i {
            Integer::Uint64(u) => u,
            Integer::Int64(i) => i as u64,
        }
    }
}
