use crate::message::Message;

mod map_ref;

pub use map_ref::MapEntryRef;

/// Maps key: message to value: message
#[derive(Debug, Clone, PartialEq)]
pub struct MapEntry {
    key: Message,
    val: Message,
}

impl MapEntry {
    /// Instantiate a new key -> value entry
    pub const fn new(key: Message, val: Message) -> Self {
        Self { key, val }
    }

    /// Underlying key
    pub const fn key(&self) -> &Message {
        &self.key
    }

    /// Underlying key with mutable access
    pub fn key_mut(&mut self) -> &mut Message {
        &mut self.key
    }

    /// Underlying value
    pub const fn val(&self) -> &Message {
        &self.val
    }

    /// Underlying value with mutable access
    pub fn val_mut(&mut self) -> &mut Message {
        &mut self.val
    }

    /// Access the internal key and value as a tuple of references
    pub const fn inner(&self) -> (&Message, &Message) {
        (&self.key, &self.val)
    }

    /// Create a new [`MapEntryRef`] with lifetime bound to `self`
    pub fn to_ref(&self) -> MapEntryRef<'_> {
        MapEntryRef::new(self.key.to_ref(), self.val.to_ref())
    }

    /// Take the internal key and value as owned value
    pub fn into_inner(self) -> (Message, Message) {
        (self.key, self.val)
    }
}
