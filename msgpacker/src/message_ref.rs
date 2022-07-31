use crate::extension::{Extension, ExtensionRef};
use crate::float::Float;
use crate::format::MessageFormat;
use crate::integer::Integer;
use crate::map::MapEntryRef;
use crate::packer::SizeableMessage;
use crate::{buffer, Message};

use std::ops::Index;
use std::time::Duration;
use std::{io, mem};

macro_rules! as_value_deref {
    ($f:ident,$r:ty,$v:ident) => {
        /// Return the attribute, if matched
        pub const fn $f(&self) -> Option<$r> {
            match self {
                Self::$v(x) => Some(*x),
                _ => None,
            }
        }
    };
}

macro_rules! as_value_ref {
    ($f:ident,$r:ty,$v:ident) => {
        /// Return the attribute, if matched
        pub fn $f(&self) -> Option<$r> {
            match self {
                Self::$v(x) => Some(x),
                _ => None,
            }
        }
    };
}

macro_rules! unpack_number {
    ($v:ident,$i:ident,$c:ident,$t:ident,$r:expr,$s:expr) => {{
        let buf = buffer::take_buf($r, $s)?;
        let n = buffer::from_slice_unchecked(&buf[..$s]);
        let n = $t::from_be_bytes(n);
        let n = $v::$i(n as $c);

        Ok(Self::$v(n))
    }};
}

/// MessagePack protocol type used as reference.
///
/// The lifetime of this struct must be bound to the `unpack` provider. If the underlying data of
/// the `reader` is dropped while this struct is still in use, some undefined behavior might
/// happen.
///
/// [specs](https://github.com/msgpack/msgpack/blob/master/spec.md#type-system)
#[derive(Debug, Clone, PartialEq)]
pub enum MessageRef<'a> {
    /// Integer 64-bit representation
    Integer(Integer),
    /// Null value
    Nil,
    /// Boolean value
    Boolean(bool),
    /// Float representation
    Float(Float),
    /// String value
    String(&'a str),
    /// Binary value
    Bin(&'a [u8]),
    /// Set of messages
    Array(Vec<Self>),
    /// Map message -> message
    Map(Vec<MapEntryRef<'a>>),
    /// Custom extension
    Extension(ExtensionRef<'a>),
}

impl<'a> MessageRef<'a> {
    /// # Safety
    ///
    /// The unsafety of this function reflects [`ExtensionRef::into_owned`]. If its safety criteria
    /// is met, then this function is safe.
    pub unsafe fn into_owned(self) -> Message {
        match self {
            Self::Integer(i) => Message::Integer(i),
            Self::Nil => Message::Nil,
            Self::Boolean(b) => Message::Boolean(b),
            Self::Float(f) => Message::Float(f),
            Self::String(s) => Message::String(s.to_owned()),
            Self::Bin(b) => Message::Bin(b.to_owned()),
            Self::Array(a) => Message::Array(a.into_iter().map(|a| a.into_owned()).collect()),
            Self::Map(m) => Message::Map(m.into_iter().map(|m| m.into_owned()).collect()),
            Self::Extension(e) => Message::Extension(e.into_owned()),
        }
    }

    as_value_deref!(as_integer, Integer, Integer);
    as_value_deref!(as_boolean, bool, Boolean);
    as_value_deref!(as_float, Float, Float);
    as_value_ref!(as_string, &'a str, String);
    as_value_ref!(as_bin, &'a [u8], Bin);
    as_value_ref!(as_array, &'a [MessageRef], Array);
    as_value_ref!(as_map, &'a [MapEntryRef], Map);
    as_value_ref!(as_extension, &'a ExtensionRef, Extension);

    /// Return `true` if the message is nil
    pub const fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    /// Create a new unsigned interger
    pub fn integer_unsigned<T>(n: T) -> Self
    where
        T: Into<u64>,
    {
        Self::Integer(Integer::unsigned(n))
    }

    /// Create a new signed interger
    pub fn integer_signed<T>(n: T) -> Self
    where
        T: Into<i64>,
    {
        Self::Integer(Integer::signed(n))
    }

    /// Create a new 32-bits floating point
    pub fn float32<F: Into<f32>>(f: F) -> Self {
        Self::Float(Float::f32(f))
    }

    /// Create a new 64-bits floating point
    pub fn float64<F: Into<f64>>(f: F) -> Self {
        Self::Float(Float::f64(f))
    }

    /// Create a new boolean
    pub fn boolean<B: Into<bool>>(b: B) -> Self {
        Self::Boolean(b.into())
    }

    /// Create a null representation
    pub const fn nil() -> Self {
        Self::Nil
    }

    /// Consume the bytes required to read a new message reference.
    ///
    /// Need to take `W` as reference until recursion resolution bug is solved:
    /// [39959](https://github.com/rust-lang/rust/issues/39959)
    ///
    /// # Safety
    ///
    /// The reader must not drop the underlying bytes before `'a` expires.
    pub unsafe fn unpack<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::BufRead,
    {
        // Fetch format
        let buf = buffer::take_buf(reader, 1)?;
        let format = MessageFormat::from(buf[0]);

        match format {
            MessageFormat::Nil => Ok(Self::Nil),

            MessageFormat::Uint8 => {
                let buf = buffer::take_buf(reader, 1)?;
                let n = Integer::Uint64(buf[0] as u64);

                Ok(Self::Integer(n))
            }

            MessageFormat::Int8 => {
                let buf = buffer::take_buf(reader, 1)?;
                let n = Integer::Int64((buf[0] as i8) as i64);

                Ok(Self::Integer(n))
            }

            MessageFormat::Uint16 => unpack_number!(Integer, Uint64, u64, u16, reader, 2),
            MessageFormat::Uint32 => unpack_number!(Integer, Uint64, u64, u32, reader, 4),
            MessageFormat::Uint64 => unpack_number!(Integer, Uint64, u64, u64, reader, 8),

            MessageFormat::Int16 => unpack_number!(Integer, Int64, i64, i16, reader, 2),
            MessageFormat::Int32 => unpack_number!(Integer, Int64, i64, i32, reader, 4),
            MessageFormat::Int64 => unpack_number!(Integer, Int64, i64, i64, reader, 8),

            MessageFormat::Float32 => unpack_number!(Float, F32, f32, f32, reader, 4),
            MessageFormat::Float64 => unpack_number!(Float, F64, f64, f64, reader, 8),

            MessageFormat::True => Ok(Self::Boolean(true)),
            MessageFormat::False => Ok(Self::Boolean(false)),

            // n-bit encoding grants range
            MessageFormat::PositiveFixint(n) => Ok(Self::Integer(Integer::Uint64(n as u64))),
            MessageFormat::NegativeFixInt(n) => Ok(Self::Integer(Integer::Int64(n as i64))),

            MessageFormat::FixStr(len) => {
                let buf = buffer::take_buf(reader, len)?;

                std::str::from_utf8(buf)
                    .map(Self::String)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            }

            MessageFormat::Str8 => {
                let buf = buffer::take_buf(reader, 1)?;
                let len = buf[0] as usize;
                let buf = buffer::take_buf(reader, len)?;

                std::str::from_utf8(buf)
                    .map(Self::String)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            }

            MessageFormat::Str16 => {
                let buf = buffer::take_buf(reader, 2)?;
                let len = buffer::from_slice_unchecked(buf);
                let len = u16::from_be_bytes(len) as usize;
                let buf = buffer::take_buf(reader, len)?;

                std::str::from_utf8(buf)
                    .map(Self::String)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            }

            MessageFormat::Str32 => {
                let buf = buffer::take_buf(reader, 4)?;
                let len = buffer::from_slice_unchecked(buf);
                let len = u32::from_be_bytes(len) as usize;
                let buf = buffer::take_buf(reader, len)?;

                std::str::from_utf8(buf)
                    .map(Self::String)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            }

            MessageFormat::Bin8 => {
                let buf = buffer::take_buf(reader, 1)?;
                let len = buf[0] as usize;
                let buf = buffer::take_buf(reader, len)?;

                Ok(Self::Bin(buf))
            }

            MessageFormat::Bin16 => {
                let buf = buffer::take_buf(reader, 2)?;
                let len = buffer::from_slice_unchecked(buf);
                let len = u16::from_be_bytes(len) as usize;
                let buf = buffer::take_buf(reader, len)?;

                Ok(Self::Bin(buf))
            }

            MessageFormat::Bin32 => {
                let buf = buffer::take_buf(reader, 4)?;
                let len = buffer::from_slice_unchecked(buf);
                let len = u32::from_be_bytes(len) as usize;
                let buf = buffer::take_buf(reader, len)?;

                Ok(Self::Bin(buf))
            }

            MessageFormat::FixArray(len) => {
                let mut arr = Vec::with_capacity(len);
                let r = reader as *mut R;

                for _ in 0..len {
                    // Safety: `unpack` is guaranteed to call `BufRead::consume` on `R`
                    let msg = Self::unpack(&mut *r)?;
                    arr.push(msg);
                }

                Ok(Self::Array(arr))
            }

            MessageFormat::Array16 => {
                let buf = buffer::take_buf(reader, 2)?;
                let len = buffer::from_slice_unchecked(buf);
                let len = u16::from_be_bytes(len) as usize;

                let mut arr = Vec::with_capacity(len);
                let r = reader as *mut R;

                for _ in 0..len {
                    // Safety: `unpack` is guaranteed to call `BufRead::consume` on `R`
                    let msg = Self::unpack(&mut *r)?;
                    arr.push(msg);
                }

                Ok(Self::Array(arr))
            }

            MessageFormat::Array32 => {
                let buf = buffer::take_buf(reader, 4)?;
                let len = buffer::from_slice_unchecked(buf);
                let len = u32::from_be_bytes(len) as usize;

                let mut arr = Vec::with_capacity(len);
                let r = reader as *mut R;

                for _ in 0..len {
                    // Safety: `unpack` is guaranteed to call `BufRead::consume` on `R`
                    let msg = Self::unpack(&mut *r)?;
                    arr.push(msg);
                }

                Ok(Self::Array(arr))
            }

            MessageFormat::FixMap(len) => {
                let mut arr = Vec::with_capacity(len);
                let r = reader as *mut R;

                for _ in 0..len {
                    // Safety: `unpack` is guaranteed to call `BufRead::consume` on `R`
                    let key = Self::unpack(&mut *r)?;
                    let val = Self::unpack(&mut *r)?;
                    let map = MapEntryRef::new(key, val);

                    arr.push(map);
                }

                Ok(Self::Map(arr))
            }

            MessageFormat::Map16 => {
                let buf = buffer::take_buf(reader, 2)?;
                let len = buffer::from_slice_unchecked(buf);
                let len = u16::from_be_bytes(len) as usize;

                let mut arr = Vec::with_capacity(len);
                let r = reader as *mut R;

                for _ in 0..len {
                    // Safety: `unpack` is guaranteed to call `BufRead::consume` on `R`
                    let key = Self::unpack(&mut *r)?;
                    let val = Self::unpack(&mut *r)?;
                    let map = MapEntryRef::new(key, val);

                    arr.push(map);
                }

                Ok(Self::Map(arr))
            }

            MessageFormat::Map32 => {
                let buf = buffer::take_buf(reader, 4)?;
                let len = buffer::from_slice_unchecked(buf);
                let len = u32::from_be_bytes(len) as usize;

                let mut arr = Vec::with_capacity(len);
                let r = reader as *mut R;

                for _ in 0..len {
                    // Safety: `unpack` is guaranteed to call `BufRead::consume` on `R`
                    let key = Self::unpack(&mut *r)?;
                    let val = Self::unpack(&mut *r)?;
                    let map = MapEntryRef::new(key, val);

                    arr.push(map);
                }

                Ok(Self::Map(arr))
            }

            MessageFormat::FixExt1 => {
                let buf = buffer::take_buf(reader, 2)?;

                Ok(Self::Extension(ExtensionRef::FixExt1(buf[0] as i8, buf[1])))
            }

            MessageFormat::FixExt2 => {
                let buf = buffer::take_buf(reader, 3)?;

                Ok(Self::Extension(ExtensionRef::FixExt2(
                    buf[0] as i8,
                    &buf[1..3],
                )))
            }

            MessageFormat::FixExt4 => {
                let buf = buffer::take_buf(reader, 5)?;
                let typ = buf[0] as i8;
                let data = &buf[1..5];

                if typ == Extension::TIMESTAMP_TYPE {
                    let data = buffer::from_slice_unchecked(data);
                    let secs = u32::from_be_bytes(data);
                    let timestamp = Duration::from_secs(secs as u64);

                    Ok(Self::Extension(ExtensionRef::Timestamp(timestamp)))
                } else {
                    Ok(Self::Extension(ExtensionRef::FixExt4(typ, data)))
                }
            }

            MessageFormat::FixExt8 => {
                let buf = buffer::take_buf(reader, 9)?;
                let typ = buf[0] as i8;
                let data = &buf[1..9];

                if typ == Extension::TIMESTAMP_TYPE {
                    let data = buffer::from_slice_unchecked(data);
                    let data = u64::from_be_bytes(data);

                    let nanos = (data >> 34) as u32;
                    let secs = data & ((1u64 << 34) - 1);

                    let timestamp = Duration::new(secs, nanos);

                    Ok(Self::Extension(ExtensionRef::Timestamp(timestamp)))
                } else {
                    Ok(Self::Extension(ExtensionRef::FixExt8(typ, data)))
                }
            }

            MessageFormat::FixExt16 => {
                let buf = buffer::take_buf(reader, 17)?;

                Ok(Self::Extension(ExtensionRef::FixExt16(
                    buf[0] as i8,
                    &buf[1..17],
                )))
            }

            MessageFormat::Ext8 => {
                let buf = buffer::take_buf(reader, 2)?;

                let len = buf[0] as usize;
                let typ = buf[1] as i8;

                if len == 12 && typ == Extension::TIMESTAMP_TYPE {
                    let buf = buffer::take_buf(reader, 12)?;

                    let nanos = buffer::from_slice_unchecked(&buf[..4]);
                    let nanos = u32::from_be_bytes(nanos);

                    let secs = buffer::from_slice_unchecked(&buf[4..12]);
                    let secs = u64::from_be_bytes(secs);

                    let timestamp = Duration::new(secs, nanos);

                    Ok(Self::Extension(ExtensionRef::Timestamp(timestamp)))
                } else {
                    let buf = buffer::take_buf(reader, len)?;

                    Ok(Self::Extension(ExtensionRef::Ext(typ, buf)))
                }
            }

            MessageFormat::Ext16 => {
                let buf = buffer::take_buf(reader, 3)?;

                let len = buffer::from_slice_unchecked(&buf[0..2]);
                let len = u16::from_be_bytes(len) as usize;

                let typ = buf[2] as i8;

                let buf = buffer::take_buf(reader, len)?;

                Ok(Self::Extension(ExtensionRef::Ext(typ, buf)))
            }

            MessageFormat::Ext32 => {
                let buf = buffer::take_buf(reader, 5)?;

                let len = buffer::from_slice_unchecked(&buf[0..4]);
                let len = u32::from_be_bytes(len) as usize;

                let typ = buf[4] as i8;

                let buf = buffer::take_buf(reader, len)?;

                Ok(Self::Extension(ExtensionRef::Ext(typ, buf)))
            }

            MessageFormat::Reserved => Err(io::Error::new(
                io::ErrorKind::Other,
                "The provided format is never used in messagepack specification!",
            )),
        }
    }

    /// Write this message as bytes into the writer, moving the cursor to the end of the written
    /// region
    ///
    /// Need to take `W` as reference until recursion resolution bug is solved:
    /// [39959](https://github.com/rust-lang/rust/issues/39959)
    pub fn pack<W>(&self, writer: &mut W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let mut n = 0;

        match self {
            Self::Nil => n += buffer::put(writer, &[u8::from(MessageFormat::Nil)])?,
            Self::Boolean(true) => n += buffer::put(writer, &[u8::from(MessageFormat::True)])?,
            Self::Boolean(false) => n += buffer::put(writer, &[u8::from(MessageFormat::False)])?,
            Self::Integer(i) => n += i.pack(writer)?,
            Self::Float(f) => n += f.pack(writer)?,

            Self::String(s) if s.len() <= 31 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixStr(s.len()))])?;
                n += buffer::put(writer, s.as_bytes())?;
            }

            Self::String(s) if s.len() <= u8::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Str8)])?;
                n += buffer::put(writer, &(s.len() as u8).to_be_bytes())?;
                n += buffer::put(writer, s.as_bytes())?;
            }

            Self::String(s) if s.len() <= u16::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Str16)])?;
                n += buffer::put(writer, &(s.len() as u16).to_be_bytes())?;
                n += buffer::put(writer, s.as_bytes())?;
            }

            Self::String(s) if s.len() <= u32::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Str32)])?;
                n += buffer::put(writer, &(s.len() as u32).to_be_bytes())?;
                n += buffer::put(writer, s.as_bytes())?;
            }

            #[allow(unreachable_patterns)]
            // Allowing unreachable_patterns for u32 usize platforms
            Self::String(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The provided string length overflows `u32` and is not allowed in messagepack!",
                ))
            }

            Self::Bin(b) if b.len() <= u8::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Bin8)])?;
                n += buffer::put(writer, &(b.len() as u8).to_be_bytes())?;
                n += buffer::put(writer, b)?;
            }

            Self::Bin(b) if b.len() <= u16::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Bin16)])?;
                n += buffer::put(writer, &(b.len() as u16).to_be_bytes())?;
                n += buffer::put(writer, b)?;
            }

            Self::Bin(b) if b.len() <= u32::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Bin32)])?;
                n += buffer::put(writer, &(b.len() as u32).to_be_bytes())?;
                n += buffer::put(writer, b)?;
            }

            #[allow(unreachable_patterns)]
            // Allowing unreachable_patterns for u32 usize platforms
            Self::Bin(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The provided binary length overflows `u32` and is not allowed in messagepack!",
                ))
            }

            Self::Array(a) if a.len() <= 15 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixArray(a.len()))])?;
                n += a
                    .iter()
                    .try_fold::<_, _, io::Result<usize>>(0, |x, m| Ok(x + m.pack(writer)?))?;
            }

            Self::Array(a) if a.len() <= u16::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Array16)])?;
                n += buffer::put(writer, &(a.len() as u16).to_be_bytes())?;
                n += a
                    .iter()
                    .try_fold::<_, _, io::Result<usize>>(0, |x, m| Ok(x + m.pack(writer)?))?;
            }

            Self::Array(a) if a.len() <= u32::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Array32)])?;
                n += buffer::put(writer, &(a.len() as u32).to_be_bytes())?;
                n += a
                    .iter()
                    .try_fold::<_, _, io::Result<usize>>(0, |x, m| Ok(x + m.pack(writer)?))?;
            }

            #[allow(unreachable_patterns)]
            // Allowing unreachable_patterns for u32 usize platforms
            Self::Array(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The provided array length overflows `u32` and is not allowed in messagepack!",
                ))
            }

            Self::Map(m) if m.len() <= 15 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixMap(m.len()))])?;
                m.iter()
                    .map(MapEntryRef::inner)
                    .try_for_each::<_, io::Result<()>>(|(k, v)| {
                        n += k.pack(writer)?;
                        n += v.pack(writer)?;

                        Ok(())
                    })?;
            }

            Self::Map(m) if m.len() <= u16::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Map16)])?;
                n += buffer::put(writer, &(m.len() as u16).to_be_bytes())?;
                m.iter()
                    .map(MapEntryRef::inner)
                    .try_for_each::<_, io::Result<()>>(|(k, v)| {
                        n += k.pack(writer)?;
                        n += v.pack(writer)?;

                        Ok(())
                    })?;
            }

            Self::Map(m) if m.len() <= u32::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Map32)])?;
                n += buffer::put(writer, &(m.len() as u32).to_be_bytes())?;
                m.iter()
                    .map(MapEntryRef::inner)
                    .try_for_each::<_, io::Result<()>>(|(k, v)| {
                        n += k.pack(writer)?;
                        n += v.pack(writer)?;

                        Ok(())
                    })?;
            }

            #[allow(unreachable_patterns)]
            // Allowing unreachable_patterns for u32 usize platforms
            Self::Map(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The provided map length overflows `u32` and is not allowed in messagepack!",
                ))
            }

            Self::Extension(ExtensionRef::FixExt1(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt1), *t as u8, *e])?;
            }

            Self::Extension(ExtensionRef::Ext(t, e)) if e.len() == 1 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt1), *t as u8, e[0]])?;
            }

            Self::Extension(ExtensionRef::FixExt2(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt2), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::Ext(t, e)) if e.len() == 2 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt2), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::FixExt4(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt4), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::Ext(t, e)) if e.len() == 4 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt4), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::FixExt8(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt8), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::Ext(t, e)) if e.len() == 8 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt8), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::FixExt16(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt16), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::Ext(t, e)) if e.len() == 16 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt16), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::Ext(t, e)) if e.len() <= u8::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Ext8)])?;
                n += buffer::put(writer, &(e.len() as u8).to_be_bytes())?;
                n += buffer::put(writer, &[*t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::Ext(t, e)) if e.len() <= u16::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Ext16)])?;
                n += buffer::put(writer, &(e.len() as u16).to_be_bytes())?;
                n += buffer::put(writer, &[*t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::Ext(t, e)) if e.len() <= u32::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Ext32)])?;
                n += buffer::put(writer, &(e.len() as u32).to_be_bytes())?;
                n += buffer::put(writer, &[*t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(ExtensionRef::Timestamp(d))
                if d.as_secs() <= u32::MAX as u64 && d.subsec_nanos() == 0 =>
            {
                n += buffer::put(
                    writer,
                    &[
                        u8::from(MessageFormat::FixExt4),
                        Extension::TIMESTAMP_TYPE as u8,
                    ],
                )?;
                n += buffer::put(writer, &(d.as_secs() as u32).to_be_bytes())?;
            }

            Self::Extension(ExtensionRef::Timestamp(d))
                if d.as_secs() < 1u64 << 34 && d.subsec_nanos() < 1u32 << 30 =>
            {
                n += buffer::put(
                    writer,
                    &[
                        u8::from(MessageFormat::FixExt8),
                        Extension::TIMESTAMP_TYPE as u8,
                    ],
                )?;

                let secs = d.as_secs();
                let secs_nanos = ((secs >> 32) & 0b11) as u32;
                let secs = secs as u32;

                let nanos = d.subsec_nanos() << 2;
                let nanos = nanos | secs_nanos;

                n += buffer::put(writer, &nanos.to_be_bytes())?;
                n += buffer::put(writer, &secs.to_be_bytes())?;
            }

            Self::Extension(ExtensionRef::Timestamp(d)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Ext8)])?;

                let nanos = d.subsec_nanos();
                let secs = d.as_secs();

                n += buffer::put(writer, &12u8.to_be_bytes())?;
                n += buffer::put(writer, &[Extension::TIMESTAMP_TYPE as u8])?;
                n += buffer::put(writer, &nanos.to_be_bytes())?;
                n += buffer::put(writer, &secs.to_be_bytes())?;
            }

            #[allow(unreachable_patterns)]
            // Allowing unreachable_patterns for u32 usize platforms
            Self::Extension(_) => return Err(io::Error::new(
                io::ErrorKind::Other,
                "The provided extension length overflows `u32` and is not allowed in messagepack!",
            )),
        }

        debug_assert_eq!(n, self.packed_len());

        Ok(n)
    }
}

impl<'a, 'b, M: Into<MessageRef<'b>>> Index<M> for MessageRef<'a> {
    type Output = MessageRef<'a>;

    fn index(&self, i: M) -> &Self::Output {
        let i = i.into();

        let m = self.as_map();
        // Safety: self is bound to 'a
        let m: Option<&'a [MapEntryRef<'a>]> = unsafe { mem::transmute(m) };

        m.and_then(|m| {
            m.iter()
                .find_map(|m| if m.key() == &i { Some(m.val()) } else { None })
        })
        .unwrap_or(&MessageRef::Nil)
    }
}

impl<'a> SizeableMessage for MessageRef<'a> {
    fn packed_len(&self) -> usize {
        match self {
            Self::Nil => 1,
            Self::Boolean(true) => 1,
            Self::Boolean(false) => 1,
            Self::Integer(i) => i.packed_len(),
            Self::Float(f) => f.packed_len(),

            Self::String(s) if s.len() <= 31 => 1 + s.len(),

            Self::String(s) if s.len() <= u8::MAX as usize => 2 + s.len(),

            Self::String(s) if s.len() <= u16::MAX as usize => 3 + s.len(),

            Self::String(s) => 5 + s.len(),

            Self::Bin(b) if b.len() <= u8::MAX as usize => 2 + b.len(),

            Self::Bin(b) if b.len() <= u16::MAX as usize => 3 + b.len(),

            Self::Bin(b) => 5 + b.len(),

            Self::Array(a) if a.len() <= 15 => 1 + a.iter().map(Self::packed_len).sum::<usize>(),

            Self::Array(a) if a.len() <= u16::MAX as usize => {
                3 + a.iter().map(Self::packed_len).sum::<usize>()
            }

            Self::Array(a) => 5 + a.iter().map(Self::packed_len).sum::<usize>(),

            Self::Map(m) if m.len() <= 15 => {
                1 + m
                    .iter()
                    .map(MapEntryRef::inner)
                    .map(|(k, v)| k.packed_len() + v.packed_len())
                    .sum::<usize>()
            }

            Self::Map(m) if m.len() <= u16::MAX as usize => {
                3 + m
                    .iter()
                    .map(MapEntryRef::inner)
                    .map(|(k, v)| k.packed_len() + v.packed_len())
                    .sum::<usize>()
            }

            Self::Map(m) => {
                4 + m
                    .iter()
                    .map(MapEntryRef::inner)
                    .map(|(k, v)| k.packed_len() + v.packed_len())
                    .sum::<usize>()
            }

            Self::Extension(ExtensionRef::FixExt1(_, _)) => 3,

            Self::Extension(ExtensionRef::Ext(_, e)) if e.len() == 1 => 3,

            Self::Extension(ExtensionRef::FixExt2(_, e)) => 2 + e.len(),

            Self::Extension(ExtensionRef::Ext(_, e)) if e.len() == 2 => 2 + e.len(),

            Self::Extension(ExtensionRef::FixExt4(_, e)) => 2 + e.len(),

            Self::Extension(ExtensionRef::Ext(_, e)) if e.len() == 4 => 2 + e.len(),

            Self::Extension(ExtensionRef::FixExt8(_, e)) => 2 + e.len(),

            Self::Extension(ExtensionRef::Ext(_, e)) if e.len() == 8 => 2 + e.len(),

            Self::Extension(ExtensionRef::FixExt16(_, e)) => 2 + e.len(),

            Self::Extension(ExtensionRef::Ext(_, e)) if e.len() == 16 => 2 + e.len(),

            Self::Extension(ExtensionRef::Ext(_, e)) if e.len() <= u8::MAX as usize => 3 + e.len(),

            Self::Extension(ExtensionRef::Ext(_, e)) if e.len() <= u16::MAX as usize => 4 + e.len(),

            Self::Extension(ExtensionRef::Ext(_, e)) => 6 + e.len(),

            Self::Extension(ExtensionRef::Timestamp(d))
                if d.as_secs() <= u32::MAX as u64 && d.subsec_nanos() == 0 =>
            {
                6
            }

            Self::Extension(ExtensionRef::Timestamp(d))
                if d.as_secs() < 1u64 << 34 && d.subsec_nanos() < 1u32 << 30 =>
            {
                10
            }

            Self::Extension(ExtensionRef::Timestamp(_)) => 15,
        }
    }
}
