use super::{
    error::Error,
    helpers::{take_buffer, take_buffer_iter, take_byte, take_byte_iter, take_num, take_num_iter},
    Format, Packable, Unpackable,
};
use alloc::{vec, vec::Vec};
use core::{iter, time::Duration};

/// Custom extension definition as reference to a bytes source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Extension {
    /// n-bytes custom extension
    Ext(i8, Vec<u8>),
    /// Protocol reserved extension to represent timestamps
    Timestamp(Duration),
}

impl Extension {
    /// Protocol constant for a timestamp extension
    pub const TIMESTAMP: i8 = -1;
}

impl Packable for Extension {
    #[allow(unreachable_code)]
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        match self {
            Extension::Ext(t, b) if b.len() == 1 => {
                buf.extend(
                    iter::once(Format::FIXEXT1)
                        .chain(iter::once(*t as u8))
                        .chain(iter::once(b[0])),
                );
                3
            }

            Extension::Ext(t, b) if b.len() == 2 => {
                buf.extend(
                    iter::once(Format::FIXEXT2)
                        .chain(iter::once(*t as u8))
                        .chain(b.iter().copied()),
                );
                4
            }

            Extension::Ext(t, b) if b.len() == 4 => {
                buf.extend(
                    iter::once(Format::FIXEXT4)
                        .chain(iter::once(*t as u8))
                        .chain(b.iter().copied()),
                );
                6
            }

            Extension::Ext(t, b) if b.len() == 8 => {
                buf.extend(
                    iter::once(Format::FIXEXT8)
                        .chain(iter::once(*t as u8))
                        .chain(b.iter().copied()),
                );
                10
            }

            Extension::Ext(t, b) if b.len() == 16 => {
                buf.extend(
                    iter::once(Format::FIXEXT16)
                        .chain(iter::once(*t as u8))
                        .chain(b.iter().copied()),
                );
                18
            }

            Extension::Ext(t, b) if b.len() <= u8::MAX as usize => {
                buf.extend(
                    iter::once(Format::EXT8)
                        .chain(iter::once(b.len() as u8))
                        .chain(iter::once(*t as u8))
                        .chain(b.iter().copied()),
                );
                3 + b.len()
            }

            Extension::Ext(t, b) if b.len() <= u16::MAX as usize => {
                buf.extend(
                    iter::once(Format::EXT16)
                        .chain((b.len() as u16).to_be_bytes().iter().copied())
                        .chain(iter::once(*t as u8))
                        .chain(b.iter().copied()),
                );
                4 + b.len()
            }

            Extension::Ext(t, b) if b.len() <= u32::MAX as usize => {
                buf.extend(
                    iter::once(Format::EXT32)
                        .chain((b.len() as u32).to_be_bytes().iter().copied())
                        .chain(iter::once(*t as u8))
                        .chain(b.iter().copied()),
                );
                6 + b.len()
            }

            Extension::Ext(_, _) => {
                #[cfg(feature = "strict")]
                panic!("strict serialization enabled; the buffer is too large");
                0
            }

            Extension::Timestamp(d) if d.as_secs() <= u32::MAX as u64 && d.subsec_nanos() == 0 => {
                buf.extend(
                    iter::once(Format::FIXEXT4)
                        .chain(iter::once(Self::TIMESTAMP as u8))
                        .chain((d.as_secs() as u32).to_be_bytes().iter().copied()),
                );
                6
            }

            Extension::Timestamp(d)
                if d.as_secs() < 1u64 << 34 && d.subsec_nanos() < 1u32 << 30 =>
            {
                let secs = d.as_secs();
                let secs_nanos = ((secs >> 32) & 0b11) as u32;
                let secs = secs as u32;

                let nanos = d.subsec_nanos() << 2;
                let nanos = nanos | secs_nanos;

                buf.extend(
                    iter::once(Format::FIXEXT8)
                        .chain(iter::once(Self::TIMESTAMP as u8))
                        .chain(nanos.to_be_bytes().iter().copied())
                        .chain(secs.to_be_bytes().iter().copied()),
                );
                10
            }

            Extension::Timestamp(d) => {
                buf.extend(
                    iter::once(Format::EXT8)
                        .chain(iter::once(12))
                        .chain(iter::once(Self::TIMESTAMP as u8))
                        .chain(d.subsec_nanos().to_be_bytes().iter().copied())
                        .chain(d.as_secs().to_be_bytes().iter().copied()),
                );
                15
            }
        }
    }
}

impl Unpackable for Extension {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            Format::FIXEXT1 => {
                let t = take_byte(&mut buf)? as i8;
                let x = take_byte(&mut buf)?;
                Ok((3, Extension::Ext(t, vec![x])))
            }
            Format::FIXEXT2 => {
                let t = take_byte(&mut buf)? as i8;
                let b = take_buffer(&mut buf, 2)?;
                Ok((4, Extension::Ext(t, b.to_vec())))
            }
            Format::FIXEXT4 => {
                let t = take_byte(&mut buf)? as i8;
                if t == Self::TIMESTAMP {
                    let secs = take_num(&mut buf, u32::from_be_bytes)?;
                    Ok((6, Extension::Timestamp(Duration::from_secs(secs as u64))))
                } else {
                    let b = take_buffer(&mut buf, 4)?;
                    Ok((6, Extension::Ext(t, b.to_vec())))
                }
            }
            Format::FIXEXT8 => {
                let t = take_byte(&mut buf)? as i8;
                if t == Self::TIMESTAMP {
                    let data = take_num(&mut buf, u64::from_be_bytes)?;

                    let nanos = (data >> 34) as u32;
                    let secs = data & ((1u64 << 34) - 1);

                    Ok((10, Extension::Timestamp(Duration::new(secs, nanos))))
                } else {
                    let b = take_buffer(&mut buf, 8)?;
                    Ok((10, Extension::Ext(t, b.to_vec())))
                }
            }
            Format::FIXEXT16 => {
                let t = take_byte(&mut buf)? as i8;
                let b = take_buffer(&mut buf, 16)?;
                Ok((18, Extension::Ext(t, b.to_vec())))
            }
            Format::EXT8 => {
                let len = take_byte(&mut buf)? as usize;
                let t = take_byte(&mut buf)? as i8;
                if len == 12 && t == Self::TIMESTAMP {
                    let nanos = take_num(&mut buf, u32::from_be_bytes)?;
                    let secs = take_num(&mut buf, u64::from_be_bytes)?;
                    Ok((15, Extension::Timestamp(Duration::new(secs, nanos))))
                } else {
                    let b = take_buffer(&mut buf, len)?;
                    Ok((3 + len, Extension::Ext(t, b.to_vec())))
                }
            }
            Format::EXT16 => {
                let len = take_num(&mut buf, u16::from_be_bytes)? as usize;
                let t = take_byte(&mut buf)? as i8;
                let b = take_buffer(&mut buf, len)?;
                Ok((4 + len, Extension::Ext(t, b.to_vec())))
            }
            Format::EXT32 => {
                let len = take_num(&mut buf, u32::from_be_bytes)? as usize;
                let t = take_byte(&mut buf)? as i8;
                let b = take_buffer(&mut buf, len)?;
                Ok((6 + len, Extension::Ext(t, b.to_vec())))
            }
            _ => Err(Error::InvalidExtension),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            Format::FIXEXT1 => {
                let t = take_byte_iter(bytes.by_ref())? as i8;
                let x = take_byte_iter(bytes.by_ref())?;
                Ok((3, Extension::Ext(t, vec![x])))
            }
            Format::FIXEXT2 => {
                let t = take_byte_iter(bytes.by_ref())? as i8;
                let b = take_buffer_iter(bytes.by_ref(), 2)?;
                Ok((4, Extension::Ext(t, b)))
            }
            Format::FIXEXT4 => {
                let t = take_byte_iter(bytes.by_ref())? as i8;
                if t == Self::TIMESTAMP {
                    let secs = take_num_iter(bytes.by_ref(), u32::from_be_bytes)?;
                    Ok((6, Extension::Timestamp(Duration::from_secs(secs as u64))))
                } else {
                    let b = take_buffer_iter(bytes.by_ref(), 4)?;
                    Ok((6, Extension::Ext(t, b)))
                }
            }
            Format::FIXEXT8 => {
                let t = take_byte_iter(bytes.by_ref())? as i8;
                if t == Self::TIMESTAMP {
                    let data = take_num_iter(bytes.by_ref(), u64::from_be_bytes)?;

                    let nanos = (data >> 34) as u32;
                    let secs = data & ((1u64 << 34) - 1);

                    Ok((10, Extension::Timestamp(Duration::new(secs, nanos))))
                } else {
                    let b = take_buffer_iter(bytes.by_ref(), 8)?;
                    Ok((10, Extension::Ext(t, b)))
                }
            }
            Format::FIXEXT16 => {
                let t = take_byte_iter(bytes.by_ref())? as i8;
                let b = take_buffer_iter(bytes.by_ref(), 16)?;
                Ok((18, Extension::Ext(t, b)))
            }
            Format::EXT8 => {
                let len = take_byte_iter(bytes.by_ref())? as usize;
                let t = take_byte_iter(bytes.by_ref())? as i8;
                if len == 12 && t == Self::TIMESTAMP {
                    let nanos = take_num_iter(bytes.by_ref(), u32::from_be_bytes)?;
                    let secs = take_num_iter(bytes.by_ref(), u64::from_be_bytes)?;
                    Ok((15, Extension::Timestamp(Duration::new(secs, nanos))))
                } else {
                    let b = take_buffer_iter(bytes.by_ref(), len)?;
                    Ok((3 + len, Extension::Ext(t, b)))
                }
            }
            Format::EXT16 => {
                let len = take_num_iter(bytes.by_ref(), u16::from_be_bytes)? as usize;
                let t = take_byte_iter(bytes.by_ref())? as i8;
                let b = take_buffer_iter(bytes.by_ref(), len)?;
                Ok((4 + len, Extension::Ext(t, b)))
            }
            Format::EXT32 => {
                let len = take_num_iter(bytes.by_ref(), u32::from_be_bytes)? as usize;
                let t = take_byte_iter(bytes.by_ref())? as i8;
                let b = take_buffer_iter(bytes.by_ref(), len)?;
                Ok((6 + len, Extension::Ext(t, b)))
            }
            _ => Err(Error::InvalidExtension),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn extension_bytes(mut t: i8, b: Vec<u8>) {
            if t == Extension::TIMESTAMP {
                t -= 1;
            }
            let x = Extension::Ext(t, b);
            let mut bytes = vec![];
            x.pack(&mut bytes);
            let (_, y) = Extension::unpack(&bytes).unwrap();
            assert_eq!(x, y);
        }

        #[test]
        fn extension_duration(d: Duration) {
            let x = Extension::Timestamp(d);
            let mut bytes = vec![];
            x.pack(&mut bytes);
            let (_, y) = Extension::unpack(&bytes).unwrap();
            assert_eq!(x, y);
        }
    }
}
