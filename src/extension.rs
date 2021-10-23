use std::slice;
use std::time::Duration;

mod extension_ref;

pub use extension_ref::ExtensionRef;

/// Custom extension definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Extension {
    /// 1 byte custom extension
    FixExt1(i8, u8),
    /// 2 bytes custom extension
    FixExt2(i8, [u8; 2]),
    /// 4 bytes custom extension
    FixExt4(i8, [u8; 4]),
    /// 8 bytes custom extension
    FixExt8(i8, [u8; 8]),
    /// 16 bytes custom extension
    FixExt16(i8, [u8; 16]),
    /// n-bytes custom extension
    Ext(i8, Vec<u8>),
    /// Protocol reserved extension to represent timestamps
    Timestamp(Duration),
}

impl Extension {
    /// Protocol reserved extension type for timestamps
    pub const TIMESTAMP_TYPE: i8 = -1;

    /// Underlying type of the extension
    pub const fn typ(&self) -> i8 {
        match self {
            Self::FixExt1(t, _) => *t,
            Self::FixExt2(t, _) => *t,
            Self::FixExt4(t, _) => *t,
            Self::FixExt8(t, _) => *t,
            Self::FixExt16(t, _) => *t,
            Self::Ext(t, _) => *t,
            Self::Timestamp(_) => Self::TIMESTAMP_TYPE,
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
            Self::Ext(_, d) => &d[..],
            Self::Timestamp(_) => &[],
        }
    }

    /// Cast a reference with lifetime bould to `self`
    pub fn to_ref(&self) -> ExtensionRef<'_> {
        match self {
            Self::FixExt1(t, e) => ExtensionRef::FixExt1(*t, *e),
            Self::FixExt2(t, e) => ExtensionRef::FixExt2(*t, e),
            Self::FixExt4(t, e) => ExtensionRef::FixExt4(*t, e),
            Self::FixExt8(t, e) => ExtensionRef::FixExt8(*t, e),
            Self::FixExt16(t, e) => ExtensionRef::FixExt16(*t, e),
            Self::Ext(t, e) => ExtensionRef::Ext(*t, e.as_slice()),
            Self::Timestamp(d) => ExtensionRef::Timestamp(*d),
        }
    }

    /// Return the protocol reserved variant for timestamp, if matches
    pub const fn timestamp(&self) -> Option<&Duration> {
        match self {
            Self::Timestamp(d) => Some(d),
            _ => None,
        }
    }
}

impl AsRef<[u8]> for Extension {
    fn as_ref(&self) -> &[u8] {
        self.data()
    }
}

impl From<Duration> for Extension {
    fn from(d: Duration) -> Self {
        Self::Timestamp(d)
    }
}
