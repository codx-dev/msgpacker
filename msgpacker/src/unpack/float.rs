use super::{
    helpers::{take_byte, take_byte_iter, take_num, take_num_iter},
    Error, Format, Unpackable,
};

impl Unpackable for f32 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        if format != Format::FLOAT32 {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((5, take_num(&mut buf, f32::from_be_bytes)?))
    }

    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut bytes = bytes.into_iter();
        let format = take_byte_iter(bytes.by_ref())?;
        if format != Format::FLOAT32 {
            return Err(Error::UnexpectedFormatTag);
        }
        Ok((5, take_num_iter(bytes.by_ref(), f32::from_be_bytes)?))
    }
}

impl Unpackable for f64 {
    type Error = Error;

    fn unpack(mut buf: &[u8]) -> Result<(usize, Self), Self::Error> {
        let format = take_byte(&mut buf)?;
        match format {
            Format::FLOAT32 => Ok((5, take_num(&mut buf, f32::from_be_bytes)? as f64)),
            Format::FLOAT64 => Ok((9, take_num(&mut buf, f64::from_be_bytes)?)),
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
            Format::FLOAT32 => Ok((5, take_num_iter(bytes.by_ref(), f32::from_be_bytes)? as f64)),
            Format::FLOAT64 => Ok((9, take_num_iter(bytes.by_ref(), f64::from_be_bytes)?)),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }
}
