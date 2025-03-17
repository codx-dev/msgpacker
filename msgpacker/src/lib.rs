#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod extension;

mod error;
mod format;
mod helpers;
mod pack;
mod unpack;

pub use error::Error;
use format::Format;
pub use pack::{pack_array, pack_map, get_array_info, pack_binary};
pub use unpack::{unpack_array, unpack_array_iter, unpack_map, unpack_map_iter};

#[cfg(feature = "alloc")]
pub use extension::Extension;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "derive")]
pub use msgpacker_derive::MsgPacker;

/// Packs the provided packable value into a vector.
#[cfg(feature = "alloc")]
pub fn pack_to_vec<T>(value: &T) -> Vec<u8>
where
    T: Packable,
{
    value.pack_to_vec()
}

/// A packable type.
pub trait Packable {
    /// Pack a value into the extendable buffer, returning the amount of written bytes.
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>;

    /// Packs the value into a vector of bytes.
    #[cfg(feature = "alloc")]
    fn pack_to_vec(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        self.pack(&mut bytes);

        bytes
    }
}

impl<X> Packable for &X
where
    X: Packable,
{
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        X::pack(self, buf)
    }
}

impl<X> Packable for &mut X
where
    X: Packable,
{
    fn pack<T>(&self, buf: &mut T) -> usize
    where
        T: Extend<u8>,
    {
        X::pack(self, buf)
    }
}

/// An unpackable type.
///
/// It provides two methods of deserialization: via slices of bytes and iterators.
///
/// Slices of bytes are more performant than iterators, but they require the bytes to be eagerly
/// loaded. If a lazy load deserialization is needed, then use `unpack_iter`.
pub trait Unpackable: Sized {
    /// Concrete error implementation for the serialization.
    ///
    /// Must interop with [Error].
    type Error: From<Error>;

    /// Unpacks a value from the buffer, returning the deserialized value and the amount of read
    /// bytes.
    fn unpack(buf: &[u8]) -> Result<(usize, Self), Self::Error>;

    /// Unpacks a value from an iterator of bytes, returning the deserialized value and the amount
    /// of read bytes.
    ///
    /// This should be used only if lazy load is required. [Unpackable::unpack] outperforms
    /// iterators with a large margin.
    fn unpack_iter<I>(bytes: I) -> Result<(usize, Self), Self::Error>
    where
        I: IntoIterator<Item = u8>;
}

/// Required types for the library.
pub mod prelude {
    pub use super::{Error, Packable, Unpackable};

    #[cfg(feature = "derive")]
    pub use super::MsgPacker;

    #[cfg(feature = "alloc")]
    pub use super::Extension;
}
