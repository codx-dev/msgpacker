use core::fmt;

/// Deserialization errors for the protocol implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// The provided buffer is too short and yielded an unexpected EOF.
    BufferTooShort,
    /// The enum variant is not valid for the static type.
    InvalidEnumVariant,
    /// The extension is not in accordance to the protocol definition.
    InvalidExtension,
    /// The string is not a valid UTF-8.
    InvalidUtf8,
    /// The protocol format tag is not valid.
    UnexpectedFormatTag,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
