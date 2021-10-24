#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod buffer;
mod extension;
mod float;
mod format;
mod integer;
mod map;
mod message;
mod message_ref;

pub use message::Message;
pub use message_ref::MessageRef;

/// Internal types of the message
pub mod types {
    pub use crate::extension::{Extension, ExtensionRef};
    pub use crate::float::Float;
    pub use crate::format::MessageFormat;
    pub use crate::integer::Integer;
    pub use crate::map::{MapEntry, MapEntryRef};
}

/// Prelude containing all public types of the crate
pub mod prelude {
    pub use crate::message::Message;
    pub use crate::message_ref::MessageRef;
    pub use crate::types::*;
}
