use crate::buffer;
use crate::extension::Extension;
use crate::float::Float;
use crate::format::MessageFormat;
use crate::integer::Integer;
use crate::map::MapEntry;
use crate::message_ref::MessageRef;

use std::io;
use std::time::Duration;

macro_rules! unpack_number {
    ($v:ident,$i:ident,$c:ident,$t:ident,$r:expr,$b:expr,$s:expr) => {{
        buffer::take($r, &mut $b, $s)?;
        let n = buffer::from_slice_unchecked(&$b[..$s]);
        let n = $t::from_be_bytes(n);
        let n = $v::$i(n as $c);

        Ok(Self::$v(n))
    }};
}

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
        pub fn $f(&self) -> Option<&$r> {
            match self {
                Self::$v(x) => Some(x),
                _ => None,
            }
        }
    };
}

fn unpack_str<R>(reader: &mut R, len: usize) -> io::Result<Message>
where
    R: io::Read,
{
    let mut val = vec![0u8; len];

    buffer::take(reader, &mut val, len)?;

    String::from_utf8(val)
        .map(Message::String)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// MessagePack protocol type
///
/// [specs](https://github.com/msgpack/msgpack/blob/master/spec.md#type-system)
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// Integer 64-bit representation
    Integer(Integer),
    /// Null value
    Nil,
    /// Boolean value
    Boolean(bool),
    /// Float representation
    Float(Float),
    /// String value
    String(String),
    /// Binary value
    Bin(Vec<u8>),
    /// Set of messages
    Array(Vec<Self>),
    /// Map message -> message
    Map(Vec<MapEntry>),
    /// Custom extension
    Extension(Extension),
}

impl Message {
    /// Cast this message to a reference variant bound to the same lifetime.
    ///
    /// The values cheaper than a pointer will be copied.
    pub fn to_ref(&self) -> MessageRef<'_> {
        match self {
            Self::Integer(i) => MessageRef::Integer(*i),
            Self::Nil => MessageRef::Nil,
            Self::Boolean(b) => MessageRef::Boolean(*b),
            Self::Float(f) => MessageRef::Float(*f),
            Self::String(s) => MessageRef::String(s.as_str()),
            Self::Bin(b) => MessageRef::Bin(b.as_slice()),
            Self::Array(a) => MessageRef::Array(a.iter().map(Self::to_ref).collect()),
            Self::Map(m) => MessageRef::Map(m.iter().map(MapEntry::to_ref).collect()),
            Self::Extension(e) => MessageRef::Extension(e.to_ref()),
        }
    }

    as_value_deref!(as_integer, Integer, Integer);
    as_value_deref!(as_boolean, bool, Boolean);
    as_value_deref!(as_float, Float, Float);
    as_value_ref!(as_string, str, String);
    as_value_ref!(as_bin, [u8], Bin);
    as_value_ref!(as_array, [Message], Array);
    as_value_ref!(as_map, [MapEntry], Map);
    as_value_ref!(as_extension, Extension, Extension);

    /// Consume the bytes required to read a new message.
    ///
    /// Need to take `R` as reference until recursion resolution bug is solved:
    /// [39959](https://github.com/rust-lang/rust/issues/39959)
    pub fn unpack<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut buf = [0u8; 32];

        // Fetch format
        buffer::take(reader, &mut buf, 1)?;
        let format = MessageFormat::from(buf[0]);

        match format {
            MessageFormat::Nil => Ok(Self::Nil),

            MessageFormat::Uint8 => {
                buffer::take(reader, &mut buf, 1)?;
                let n = Integer::Uint64(buf[0] as u64);

                Ok(Self::Integer(n))
            }

            MessageFormat::Int8 => {
                buffer::take(reader, &mut buf, 1)?;
                let n = Integer::Int64(buf[0] as i64);

                Ok(Self::Integer(n))
            }

            MessageFormat::Uint16 => unpack_number!(Integer, Uint64, u64, u16, reader, buf, 2),
            MessageFormat::Uint32 => unpack_number!(Integer, Uint64, u64, u32, reader, buf, 4),
            MessageFormat::Uint64 => unpack_number!(Integer, Uint64, u64, u64, reader, buf, 8),

            MessageFormat::Int16 => unpack_number!(Integer, Int64, i64, i16, reader, buf, 2),
            MessageFormat::Int32 => unpack_number!(Integer, Int64, i64, i32, reader, buf, 4),
            MessageFormat::Int64 => unpack_number!(Integer, Int64, i64, i64, reader, buf, 8),

            MessageFormat::Float32 => unpack_number!(Float, F32, f32, f32, reader, buf, 4),
            MessageFormat::Float64 => unpack_number!(Float, F64, f64, f64, reader, buf, 8),

            MessageFormat::True => Ok(Self::Boolean(true)),
            MessageFormat::False => Ok(Self::Boolean(false)),

            // n-bit encoding grants range
            MessageFormat::PositiveFixint(n) => Ok(Self::Integer(Integer::Uint64(n as u64))),
            MessageFormat::NegativeFixInt(n) => Ok(Self::Integer(Integer::Int64(n as i64))),

            MessageFormat::FixStr(len) => unpack_str(reader, len),

            MessageFormat::Str8 => {
                buffer::take(reader, &mut buf, 1)?;
                let len = buf[0] as usize;

                unpack_str(reader, len)
            }

            MessageFormat::Str16 => {
                buffer::take(reader, &mut buf, 2)?;
                let len = buffer::from_slice_unchecked(&buf[..2]);
                let len = u16::from_be_bytes(len) as usize;

                unpack_str(reader, len)
            }

            MessageFormat::Str32 => {
                buffer::take(reader, &mut buf, 4)?;
                let len = buffer::from_slice_unchecked(&buf[..4]);
                let len = u32::from_be_bytes(len) as usize;

                unpack_str(reader, len)
            }

            MessageFormat::Bin8 => {
                buffer::take(reader, &mut buf, 1)?;
                let len = buf[0] as usize;

                let mut val = vec![0u8; len];
                buffer::take(reader, &mut val, len)?;

                Ok(Self::Bin(val))
            }

            MessageFormat::Bin16 => {
                buffer::take(reader, &mut buf, 2)?;
                let len = buffer::from_slice_unchecked(&buf[..2]);
                let len = u16::from_be_bytes(len) as usize;

                let mut val = vec![0u8; len];
                buffer::take(reader, &mut val, len)?;

                Ok(Self::Bin(val))
            }

            MessageFormat::Bin32 => {
                buffer::take(reader, &mut buf, 4)?;
                let len = buffer::from_slice_unchecked(&buf[..4]);
                let len = u32::from_be_bytes(len) as usize;

                let mut val = vec![0u8; len];
                buffer::take(reader, &mut val, len)?;

                Ok(Self::Bin(val))
            }

            MessageFormat::FixArray(len) => (0..len)
                .map(|_| Self::unpack(reader))
                .collect::<io::Result<Vec<Self>>>()
                .map(Self::Array),

            MessageFormat::Array16 => {
                buffer::take(reader, &mut buf, 2)?;
                let len = buffer::from_slice_unchecked(&buf[..2]);
                let len = u16::from_be_bytes(len) as usize;

                (0..len)
                    .map(|_| Self::unpack(reader))
                    .collect::<io::Result<Vec<Self>>>()
                    .map(Self::Array)
            }

            MessageFormat::Array32 => {
                buffer::take(reader, &mut buf, 4)?;
                let len = buffer::from_slice_unchecked(&buf[..4]);
                let len = u32::from_be_bytes(len) as usize;

                (0..len)
                    .map(|_| Self::unpack(reader))
                    .collect::<io::Result<Vec<Self>>>()
                    .map(Self::Array)
            }

            MessageFormat::FixMap(len) => (0..len)
                .map(|_| {
                    let key = Self::unpack(reader)?;
                    let val = Self::unpack(reader)?;

                    Ok(MapEntry::new(key, val))
                })
                .collect::<io::Result<Vec<MapEntry>>>()
                .map(Self::Map),

            MessageFormat::Map16 => {
                buffer::take(reader, &mut buf, 2)?;
                let len = buffer::from_slice_unchecked(&buf[..2]);
                let len = u16::from_be_bytes(len) as usize;

                (0..len)
                    .map(|_| {
                        let key = Self::unpack(reader)?;
                        let val = Self::unpack(reader)?;

                        Ok(MapEntry::new(key, val))
                    })
                    .collect::<io::Result<Vec<MapEntry>>>()
                    .map(Self::Map)
            }

            MessageFormat::Map32 => {
                buffer::take(reader, &mut buf, 4)?;
                let len = buffer::from_slice_unchecked(&buf[..4]);
                let len = u32::from_be_bytes(len) as usize;

                (0..len)
                    .map(|_| {
                        let key = Self::unpack(reader)?;
                        let val = Self::unpack(reader)?;

                        Ok(MapEntry::new(key, val))
                    })
                    .collect::<io::Result<Vec<MapEntry>>>()
                    .map(Self::Map)
            }

            MessageFormat::FixExt1 => {
                buffer::take(reader, &mut buf, 2)?;

                Ok(Self::Extension(Extension::FixExt1(buf[0] as i8, buf[1])))
            }

            MessageFormat::FixExt2 => {
                buffer::take(reader, &mut buf, 3)?;
                let data = buffer::from_slice_unchecked(&buf[1..3]);

                Ok(Self::Extension(Extension::FixExt2(buf[0] as i8, data)))
            }

            MessageFormat::FixExt4 => {
                buffer::take(reader, &mut buf, 5)?;
                let typ = buf[0] as i8;
                let data = buffer::from_slice_unchecked(&buf[1..5]);

                if typ == Extension::TIMESTAMP_TYPE {
                    let secs = u32::from_be_bytes(data);
                    let timestamp = Duration::from_secs(secs as u64);

                    Ok(Self::Extension(Extension::Timestamp(timestamp)))
                } else {
                    Ok(Self::Extension(Extension::FixExt4(typ, data)))
                }
            }

            MessageFormat::FixExt8 => {
                buffer::take(reader, &mut buf, 9)?;
                let typ = buf[0] as i8;
                let data = buffer::from_slice_unchecked(&buf[1..9]);

                if typ == Extension::TIMESTAMP_TYPE {
                    let data = u64::from_be_bytes(data);

                    let nanos = (data >> 34) as u32;
                    let secs = data & ((1u64 << 34) - 1);

                    let timestamp = Duration::new(secs, nanos);

                    Ok(Self::Extension(Extension::Timestamp(timestamp)))
                } else {
                    Ok(Self::Extension(Extension::FixExt8(typ, data)))
                }
            }

            MessageFormat::FixExt16 => {
                buffer::take(reader, &mut buf, 17)?;
                let data = buffer::from_slice_unchecked(&buf[1..17]);

                Ok(Self::Extension(Extension::FixExt16(buf[0] as i8, data)))
            }

            MessageFormat::Ext8 => {
                buffer::take(reader, &mut buf, 2)?;

                let len = buf[0] as usize;
                let typ = buf[1] as i8;

                if len == 12 && typ == Extension::TIMESTAMP_TYPE {
                    buffer::take(reader, &mut buf, 12)?;

                    let nanos = buffer::from_slice_unchecked(&buf[..4]);
                    let nanos = u32::from_be_bytes(nanos);

                    let secs = buffer::from_slice_unchecked(&buf[4..12]);
                    let secs = u64::from_be_bytes(secs);

                    let timestamp = Duration::new(secs, nanos);

                    Ok(Self::Extension(Extension::Timestamp(timestamp)))
                } else {
                    let mut val = vec![0u8; len];

                    buffer::take(reader, &mut val, len)?;
                    Ok(Self::Extension(Extension::Ext(typ, val)))
                }
            }

            MessageFormat::Ext16 => {
                buffer::take(reader, &mut buf, 3)?;

                let len = buffer::from_slice_unchecked(&buf[0..2]);
                let len = u16::from_be_bytes(len) as usize;

                let typ = buf[2] as i8;

                let mut val = vec![0u8; len];

                buffer::take(reader, &mut val, len)?;
                Ok(Self::Extension(Extension::Ext(typ, val)))
            }

            MessageFormat::Ext32 => {
                buffer::take(reader, &mut buf, 5)?;

                let len = buffer::from_slice_unchecked(&buf[0..4]);
                let len = u32::from_be_bytes(len) as usize;

                let typ = buf[4] as i8;

                let mut val = vec![0u8; len];

                buffer::take(reader, &mut val, len)?;
                Ok(Self::Extension(Extension::Ext(typ, val)))
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
    /// [specs](https://github.com/rust-lang/rust/issues/39959)
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
                n += buffer::put(writer, b.as_slice())?;
            }

            Self::Bin(b) if b.len() <= u16::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Bin16)])?;
                n += buffer::put(writer, &(b.len() as u16).to_be_bytes())?;
                n += buffer::put(writer, b.as_slice())?;
            }

            Self::Bin(b) if b.len() <= u32::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Bin32)])?;
                n += buffer::put(writer, &(b.len() as u32).to_be_bytes())?;
                n += buffer::put(writer, b.as_slice())?;
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
                    .map(MapEntry::inner)
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
                    .map(MapEntry::inner)
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
                    .map(MapEntry::inner)
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

            Self::Extension(Extension::FixExt1(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt1), *t as u8, *e])?;
            }

            Self::Extension(Extension::Ext(t, e)) if e.len() == 1 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt1), *t as u8, e[0]])?;
            }

            Self::Extension(Extension::FixExt2(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt2), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(Extension::Ext(t, e)) if e.len() == 2 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt2), *t as u8])?;
                n += buffer::put(writer, e.as_slice())?;
            }

            Self::Extension(Extension::FixExt4(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt4), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(Extension::Ext(t, e)) if e.len() == 4 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt4), *t as u8])?;
                n += buffer::put(writer, e.as_slice())?;
            }

            Self::Extension(Extension::FixExt8(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt8), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(Extension::Ext(t, e)) if e.len() == 8 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt8), *t as u8])?;
                n += buffer::put(writer, e.as_slice())?;
            }

            Self::Extension(Extension::FixExt16(t, e)) => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt16), *t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(Extension::Ext(t, e)) if e.len() == 16 => {
                n += buffer::put(writer, &[u8::from(MessageFormat::FixExt16), *t as u8])?;
                n += buffer::put(writer, e.as_slice())?;
            }

            Self::Extension(Extension::Ext(t, e)) if e.len() <= u8::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Ext8)])?;
                n += buffer::put(writer, &(e.len() as u8).to_be_bytes())?;
                n += buffer::put(writer, &[*t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(Extension::Ext(t, e)) if e.len() <= u16::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Ext16)])?;
                n += buffer::put(writer, &(e.len() as u16).to_be_bytes())?;
                n += buffer::put(writer, &[*t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(Extension::Ext(t, e)) if e.len() <= u32::MAX as usize => {
                n += buffer::put(writer, &[u8::from(MessageFormat::Ext32)])?;
                n += buffer::put(writer, &(e.len() as u32).to_be_bytes())?;
                n += buffer::put(writer, &[*t as u8])?;
                n += buffer::put(writer, e)?;
            }

            Self::Extension(Extension::Timestamp(d))
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

            Self::Extension(Extension::Timestamp(d))
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
                let secs_nanos = (secs >> 32) as u32;
                let secs = secs as u32;

                let nanos = d.subsec_nanos() << 2;
                let nanos = nanos | secs_nanos;

                n += buffer::put(writer, &nanos.to_be_bytes())?;
                n += buffer::put(writer, &secs.to_be_bytes())?;
            }

            Self::Extension(Extension::Timestamp(d)) => {
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

        Ok(n)
    }
}
