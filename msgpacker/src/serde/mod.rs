//! [serde] implementations.

use serde::{Deserialize, Serialize};

use crate::Error;

mod deserializer;
mod serializer;

/// Serializes the provided value into the extendable buffer.
///
/// This operation is infallible as it will only allocate bytes.
pub fn to_buffer<X, T>(buffer: &mut X, value: &T)
where
    X: Extend<u8>,
    T: Serialize + ?Sized,
{
    value
        .serialize(&mut serializer::MsgpackSerializer::from(buffer))
        .ok();
}

/// Serializes the provided value into a [Vec<u8>].
#[cfg(feature = "alloc")]
pub fn to_vec<T>(value: &T) -> ::alloc::vec::Vec<u8>
where
    T: Serialize + ?Sized,
{
    let mut v = Vec::new();

    to_buffer(&mut v, value);

    v
}

/// Deserializes the data from the given slice.
pub fn from_slice<'a, T>(s: &'a [u8]) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    T::deserialize(&mut deserializer::MsgpackDeserializer(s))
}
