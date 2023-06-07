use super::{
    helpers::{take_byte, take_byte_iter, take_num, take_num_iter},
    Error, Format, Unpackable,
};

impl Unpackable for u8 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for u16 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u16)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as u16)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u16)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as u16)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for u32 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u32)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as u32)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v as u32)),
            Format::UINT32 => take_num(&mut buf, u32::from_be_bytes).map(|v| (5, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u32)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as u32)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v as u32)),
            Format::UINT32 => take_num_iter(bytes, u32::from_be_bytes).map(|v| (5, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for u64 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u64)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as u64)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v as u64)),
            Format::UINT32 => take_num(&mut buf, u32::from_be_bytes).map(|v| (5, v as u64)),
            Format::UINT64 => take_num(&mut buf, u64::from_be_bytes).map(|v| (9, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as u64)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as u64)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v as u64)),
            Format::UINT32 => take_num_iter(bytes, u32::from_be_bytes).map(|v| (3, v as u64)),
            Format::UINT64 => take_num_iter(bytes, u64::from_be_bytes).map(|v| (9, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for usize {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as usize)),
            Format::UINT8 => take_byte(&mut buf).map(|v| (2, v as usize)),
            Format::UINT16 => take_num(&mut buf, u16::from_be_bytes).map(|v| (3, v as usize)),
            Format::UINT32 => take_num(&mut buf, u32::from_be_bytes).map(|v| (5, v as usize)),
            Format::UINT64 => take_num(&mut buf, u64::from_be_bytes).map(|v| (9, v as usize)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as usize)),
            Format::UINT8 => take_byte_iter(bytes).map(|v| (2, v as usize)),
            Format::UINT16 => take_num_iter(bytes, u16::from_be_bytes).map(|v| (3, v as usize)),
            Format::UINT32 => take_num_iter(bytes, u32::from_be_bytes).map(|v| (3, v as usize)),
            Format::UINT64 => take_num_iter(bytes, usize::from_be_bytes).map(|v| (9, v)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for i8 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as i8)),
            0xe0.. => Ok((1, format as i8)),
            Format::INT8 => Ok((2, take_byte(&mut buf)? as i8)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, format as i8)),
            0xe0.. => Ok((1, format as i8)),
            Format::INT8 => Ok((2, take_byte_iter(bytes)? as i8)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for i16 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i16)),
            0xe0.. => Ok((1, (format as i8) as i16)),
            Format::INT8 => Ok((2, take_byte(&mut buf)? as i8 as i16)),
            Format::INT16 => Ok((3, take_num(&mut buf, i16::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i16)),
            0xe0.. => Ok((1, (format as i8) as i16)),
            Format::INT8 => Ok((2, take_byte_iter(bytes)? as i8 as i16)),
            Format::INT16 => Ok((3, take_num_iter(bytes, i16::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for i32 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i32)),
            0xe0.. => Ok((1, (format as i8) as i32)),
            Format::INT8 => Ok((2, take_byte(&mut buf)? as i8 as i32)),
            Format::INT16 => Ok((3, take_num(&mut buf, i16::from_be_bytes)? as i32)),
            Format::INT32 => Ok((5, take_num(&mut buf, i32::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i32)),
            0xe0.. => Ok((1, (format as i8) as i32)),
            Format::INT8 => Ok((2, take_byte_iter(bytes)? as i8 as i32)),
            Format::INT16 => Ok((3, take_num_iter(bytes, i16::from_be_bytes)? as i32)),
            Format::INT32 => Ok((5, take_num_iter(bytes, i32::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for i64 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i64)),
            0xe0.. => Ok((1, (format as i8) as i64)),
            Format::INT8 => Ok((2, take_byte(&mut buf)? as i8 as i64)),
            Format::INT16 => Ok((3, take_num(&mut buf, i16::from_be_bytes)? as i64)),
            Format::INT32 => Ok((5, take_num(&mut buf, i32::from_be_bytes)? as i64)),
            Format::INT64 => Ok((9, take_num(&mut buf, i64::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as i64)),
            0xe0.. => Ok((1, (format as i8) as i64)),
            Format::INT8 => Ok((2, take_byte_iter(bytes)? as i8 as i64)),
            Format::INT16 => Ok((3, take_num_iter(bytes, i16::from_be_bytes)? as i64)),
            Format::INT32 => Ok((5, take_num_iter(bytes, i32::from_be_bytes)? as i64)),
            Format::INT64 => Ok((9, take_num_iter(bytes, i64::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}

impl Unpackable for isize {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as isize)),
            0xe0.. => Ok((1, (format as i8) as isize)),
            Format::INT8 => Ok((2, take_byte(&mut buf)? as i8 as isize)),
            Format::INT16 => Ok((3, take_num(&mut buf, i16::from_be_bytes)? as isize)),
            Format::INT32 => Ok((5, take_num(&mut buf, i32::from_be_bytes)? as isize)),
            Format::INT64 => Ok((9, take_num(&mut buf, isize::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        match format {
            0x00..=Format::POSITIVE_FIXINT => Ok((1, (format as i8) as isize)),
            0xe0.. => Ok((1, (format as i8) as isize)),
            Format::INT8 => Ok((2, take_byte_iter(bytes)? as i8 as isize)),
            Format::INT16 => Ok((3, take_num_iter(bytes, i16::from_be_bytes)? as isize)),
            Format::INT32 => Ok((5, take_num_iter(bytes, i32::from_be_bytes)? as isize)),
            Format::INT64 => Ok((9, take_num_iter(bytes, isize::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}
