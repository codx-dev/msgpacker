use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;

mod impls;

/// Messages that have a defined packed length
pub trait SizeableMessage {
    /// Packed length of the message
    fn packed_len(&self) -> usize;
}

/// Define a type that can be packed into a writer.
pub trait Packable: Sized + SizeableMessage {
    /// Pack the type into a writer
    fn pack<W>(&self, packer: W) -> io::Result<usize>
    where
        W: Write;
}

/// Define a type that can be unpacked from a reader.
pub trait Unpackable: Sized {
    /// Unpack the type from a reader
    fn unpack<R>(unpacker: R) -> io::Result<Self>
    where
        R: BufRead;
}

/// A packer implementation that can receive messages
pub trait MessagePacker {
    /// Pack the given argument into the internal writer
    fn pack<P>(&mut self, package: P) -> io::Result<usize>
    where
        P: Packable;

    /// Use the packer by ref
    fn by_ref(&mut self) -> &mut Self {
        self
    }

    /// Pack all items of the iterator
    fn pack_all<I, P>(&mut self, mut iter: I) -> io::Result<usize>
    where
        P: Packable,
        I: Iterator<Item = P>,
    {
        iter.try_fold(0, |n, p| Ok(n + self.pack(p)?))
    }
}

#[cfg(feature = "impl-io")]
impl<W> MessagePacker for W
where
    W: io::Write,
{
    fn pack<P>(&mut self, package: P) -> io::Result<usize>
    where
        P: Packable,
    {
        package.pack(self)
    }
}

/// An unpacker implementation that can output messages
pub trait MessageUnpacker {
    /// Unpack a message from the internal reader
    fn unpack<P>(&mut self) -> io::Result<P>
    where
        P: Unpackable;

    /// Use the unpacker by ref
    fn by_ref(&mut self) -> &mut Self {
        self
    }
}

#[cfg(feature = "impl-io")]
impl<R> MessageUnpacker for R
where
    R: io::Read,
{
    fn unpack<P>(&mut self) -> io::Result<P>
    where
        P: Unpackable,
    {
        let reader = io::BufReader::new(self);

        P::unpack(reader)
    }
}

/// Iterator helper of unpackable items
///
/// # Warning
///
/// The iterator will provide messages until [`io::ErrorKind::UnexpectedEof`].
///
/// If EOF happens in the middle of a valid message, this case will be ignored and the iterator
/// will return `None`. This exception doesn't build a strong enough rule to compromise the
/// ergonomics of the struct.
///
/// We might, in the future, create a `SizedMessageUnpacker` so we can trivially check if EOF
/// corresponds to the expected EOF. However, in most use-cases, the underlying buffer will be of
/// unknown size so this might not be used at all.
pub struct UnpackableIter<I, P>
where
    I: MessageUnpacker,
    P: Unpackable,
{
    unpacker: I,
    _package: PhantomData<P>,
}

impl<I, P> From<I> for UnpackableIter<I, P>
where
    I: MessageUnpacker,
    P: Unpackable,
{
    fn from(unpacker: I) -> Self {
        Self {
            unpacker,
            _package: PhantomData,
        }
    }
}

impl<R, P> From<R> for UnpackableIter<io::BufReader<R>, P>
where
    R: Read,
    P: Unpackable,
{
    fn from(reader: R) -> Self {
        io::BufReader::new(reader).into()
    }
}

impl<I, P> UnpackableIter<I, P>
where
    I: MessageUnpacker,
    P: Unpackable,
{
    /// Unpack messages until EOF is hit
    pub fn unpack_until_eof(&mut self) -> io::Result<Vec<P>>
    where
        P: Unpackable,
    {
        self.try_fold(vec![], |mut v, p| {
            v.push(p?);

            Ok(v)
        })
    }
}

impl<P> UnpackableIter<io::BufReader<fs::File>, P>
where
    P: Unpackable,
{
    /// Open a file as read-only to extract unpackable items
    pub fn open_file<F>(path: F) -> io::Result<Self>
    where
        F: AsRef<Path>,
    {
        fs::OpenOptions::new().read(true).open(path).map(Self::from)
    }
}

impl<I, P> Deref for UnpackableIter<I, P>
where
    I: MessageUnpacker,
    P: Unpackable,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.unpacker
    }
}

impl<I, P> DerefMut for UnpackableIter<I, P>
where
    I: MessageUnpacker,
    P: Unpackable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.unpacker
    }
}

impl<I, P> Iterator for UnpackableIter<I, P>
where
    I: MessageUnpacker,
    P: Unpackable,
{
    type Item = io::Result<P>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.unpack::<P>() {
            Ok(p) => Some(Ok(p)),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}
