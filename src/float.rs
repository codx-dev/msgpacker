use crate::buffer;
use crate::format::MessageFormat;

use std::io;

macro_rules! as_float {
    ($f:ident,$r:ident,$v:ident) => {
        /// Return the underlying value, if matches
        pub const fn $f(&self) -> Option<$r> {
            match self {
                Self::$v(n) => Some(*n),
                _ => None,
            }
        }
    };
}

/// [specs](https://github.com/msgpack/msgpack/blob/master/spec.md#float-format-family)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Float {
    /// 32-bits float
    F32(f32),
    /// 64-bits float
    F64(f64),
}

impl Float {
    as_float!(as_f32, f32, F32);
    as_float!(as_f64, f64, F64);

    /// Pack this float into writer and return the amount of bytes written
    pub fn pack<W>(&self, mut writer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let mut n = 0;

        match self {
            Self::F32(f) => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Float32)])?;
                n += buffer::put(&mut writer, &f.to_be_bytes())?;
            }

            Self::F64(f) => {
                n += buffer::put(&mut writer, &[u8::from(MessageFormat::Float64)])?;
                n += buffer::put(&mut writer, &f.to_be_bytes())?;
            }
        }

        Ok(n)
    }
}
