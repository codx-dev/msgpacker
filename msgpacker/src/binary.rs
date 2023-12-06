use core::ops::Deref;

/// Wrapper struct to mark [u8] that are packed as bin rather than array
#[derive(Debug, PartialEq)]
pub struct MsgPackerBinSlice<'a>(pub &'a [u8]);

impl<'a> Deref for MsgPackerBinSlice<'a> {
    type Target = [u8];

    fn deref(&self) -> &'a Self::Target {
        self.0
    }
}

#[cfg(feature = "alloc")]
pub mod alloc {
    use super::*;
    use ::alloc::vec::Vec;

    /// Wrapper struct to mark Vec<u8> that are packed as bin rather than array
    #[derive(Clone, Debug, PartialEq)]
    pub struct MsgPackerBin(pub Vec<u8>);

    impl MsgPackerBin {
        /// Extracts a MsgPackerBinSlice containing the entire MsgPackerBin.
        pub fn as_slice(&self) -> MsgPackerBinSlice {
            MsgPackerBinSlice(self.0.as_slice())
        }
    }

    impl Deref for MsgPackerBin {
        type Target = Vec<u8>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
