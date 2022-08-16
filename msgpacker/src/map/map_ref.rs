use super::MapEntry;
use crate::message_ref::MessageRef;

/// Maps key: message to value: message with lifetime bound to `MessageRef`
#[derive(Debug, Clone, PartialEq)]
pub struct MapEntryRef<'a> {
    key: MessageRef<'a>,
    val: MessageRef<'a>,
}

impl<'a> MapEntryRef<'a> {
    /// Instantiate a new key -> value entry
    pub const fn new(key: MessageRef<'a>, val: MessageRef<'a>) -> Self {
        Self { key, val }
    }

    /// Underlying key
    pub const fn key(&self) -> &MessageRef<'a> {
        &self.key
    }

    /// Underlying value
    pub const fn val(&self) -> &MessageRef<'a> {
        &self.val
    }

    pub(crate) fn into_inner(self) -> (MessageRef<'a>, MessageRef<'a>) {
        (self.key, self.val)
    }

    /// Access the internal key and value as a tuple of references
    pub const fn inner(&self) -> (&MessageRef<'a>, &MessageRef<'a>) {
        (&self.key, &self.val)
    }

    /// # Safety
    ///
    /// The unsafety of this function reflects [`crate::types::ExtensionRef::into_owned`].
    ///
    /// If its safety criteria is met, then this function is safe.
    pub unsafe fn into_owned(self) -> MapEntry {
        MapEntry::new(self.key.into_owned(), self.val.into_owned())
    }
}
