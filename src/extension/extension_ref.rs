use super::Extension;
use crate::buffer;

use std::slice;
use std::time::Duration;

/// Custom extension definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtensionRef<'a> {
    /// 1 byte custom extension
    FixExt1(i8, u8),
    /// 2 bytes custom extension
    FixExt2(i8, &'a [u8]),
    /// 4 bytes custom extension
    FixExt4(i8, &'a [u8]),
    /// 8 bytes custom extension
    FixExt8(i8, &'a [u8]),
    /// 16 bytes custom extension
    FixExt16(i8, &'a [u8]),
    /// n-bytes custom extension
    Ext(i8, &'a [u8]),
    /// Protocol reserved extension to represent timestamps
    Timestamp(Duration),
}

impl<'a> ExtensionRef<'a> {
    /// Underlying type of the extension
    pub const fn typ(&self) -> i8 {
        match self {
            Self::FixExt1(t, _) => *t,
            Self::FixExt2(t, _) => *t,
            Self::FixExt4(t, _) => *t,
            Self::FixExt8(t, _) => *t,
            Self::FixExt16(t, _) => *t,
            Self::Ext(t, _) => *t,
            Self::Timestamp(_) => Extension::TIMESTAMP_TYPE,
        }
    }

    /// Underlying data of the extension
    pub fn data(&self) -> &[u8] {
        match self {
            Self::FixExt1(_, d) => slice::from_ref(d),
            Self::FixExt2(_, d) => d,
            Self::FixExt4(_, d) => d,
            Self::FixExt8(_, d) => d,
            Self::FixExt16(_, d) => d,
            Self::Ext(_, d) => d,
            Self::Timestamp(_) => &[],
        }
    }

    /// Return the protocol reserved variant for timestamp, if matches
    pub const fn timestamp(&self) -> Option<&Duration> {
        match self {
            Self::Timestamp(d) => Some(d),
            _ => None,
        }
    }

    /// # Safety
    ///
    /// May result in undefined behavior if the fixed extensions point to invalid slices. If these
    /// slices are valid, the function is safe.
    pub unsafe fn into_owned(self) -> Extension {
        match self {
            Self::FixExt1(t, d) => Extension::FixExt1(t, d),
            Self::FixExt2(t, d) => Extension::FixExt2(t, buffer::from_slice_unchecked(d)),
            Self::FixExt4(t, d) => Extension::FixExt4(t, buffer::from_slice_unchecked(d)),
            Self::FixExt8(t, d) => Extension::FixExt8(t, buffer::from_slice_unchecked(d)),
            Self::FixExt16(t, d) => Extension::FixExt16(t, buffer::from_slice_unchecked(d)),
            Self::Ext(t, d) => Extension::Ext(t, d.to_owned()),
            Self::Timestamp(t) => Extension::Timestamp(t),
        }
    }
}

impl<'a> AsRef<[u8]> for ExtensionRef<'a> {
    fn as_ref(&self) -> &[u8] {
        self.data()
    }
}

impl<'a> From<Duration> for ExtensionRef<'a> {
    fn from(d: Duration) -> ExtensionRef<'a> {
        Self::Timestamp(d)
    }
}
