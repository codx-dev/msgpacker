use std::fs;
use std::io::{self, BufRead, Cursor, Read, Seek, SeekFrom, Write};
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

impl<M> MessagePacker for &mut M
where
    M: MessagePacker,
{
    fn pack<P>(&mut self, package: P) -> io::Result<usize>
    where
        P: Packable,
    {
        <M as MessagePacker>::pack(self, package)
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

impl<M> MessageUnpacker for &mut M
where
    M: MessageUnpacker,
{
    fn unpack<P>(&mut self) -> io::Result<P>
    where
        P: Unpackable,
    {
        <M as MessageUnpacker>::unpack(self)
    }
}

impl<R> MessageUnpacker for io::BufReader<R>
where
    R: io::Read,
{
    fn unpack<P>(&mut self) -> io::Result<P>
    where
        P: Unpackable,
    {
        P::unpack(self)
    }
}

/// A buffered packer implementation
#[derive(Debug)]
pub struct BufferedUnpacker<R> {
    buffer: io::BufReader<R>,
}

impl<R> BufferedUnpacker<R>
where
    R: io::Read,
{
    /// Create a new instance from an implementation of [`io::Read`]
    pub fn from_reader(reader: R) -> Self {
        io::BufReader::new(reader).into()
    }
}

impl<R> From<io::BufReader<R>> for BufferedUnpacker<R> {
    fn from(buffer: io::BufReader<R>) -> Self {
        Self { buffer }
    }
}

impl<R> Deref for BufferedUnpacker<R> {
    type Target = io::BufReader<R>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<R> DerefMut for BufferedUnpacker<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl<R> Read for BufferedUnpacker<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl<R> io::Seek for BufferedUnpacker<R>
where
    R: io::Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.buffer.seek(pos)
    }
}

impl<R> io::BufRead for BufferedUnpacker<R>
where
    R: io::Read,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.buffer.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.buffer.consume(amt)
    }
}

impl<R> MessageUnpacker for BufferedUnpacker<R>
where
    R: io::Read,
{
    fn unpack<P>(&mut self) -> io::Result<P>
    where
        P: Unpackable,
    {
        P::unpack(self)
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

impl MessagePacker for fs::File {
    fn pack<P>(&mut self, package: P) -> io::Result<usize>
    where
        P: Packable,
    {
        package.pack(self)
    }
}

/// A packer/unpacker implementation with an underlying [`Cursor`]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CursorPacker<B> {
    cursor: Cursor<B>,
}

impl<B> Deref for CursorPacker<B> {
    type Target = Cursor<B>;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl<B> DerefMut for CursorPacker<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cursor
    }
}

impl<B> From<Cursor<B>> for CursorPacker<B> {
    fn from(cursor: Cursor<B>) -> Self {
        Self { cursor }
    }
}

impl<B> From<CursorPacker<B>> for Cursor<B> {
    fn from(packer: CursorPacker<B>) -> Self {
        packer.cursor
    }
}

impl<B> CursorPacker<B> {
    /// Create a new cursor packer with an underlying [`Cursor`]
    pub fn new(buffer: B) -> Self {
        Cursor::new(buffer).into()
    }
}

impl CursorPacker<Vec<u8>> {
    /// Read file into buffer
    pub fn from_file<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        fs::read(path).map(Self::new)
    }
}

impl<B> Read for CursorPacker<B>
where
    B: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cursor.read(buf)
    }
}

impl<B> MessageUnpacker for CursorPacker<B>
where
    B: AsRef<[u8]>,
{
    fn unpack<P>(&mut self) -> io::Result<P>
    where
        P: Unpackable,
    {
        P::unpack(self)
    }
}

macro_rules! impl_packer_writer {
    ($b:ty) => {
        impl Write for CursorPacker<$b> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.cursor.write(buf)
            }

            fn flush(&mut self) -> io::Result<()> {
                self.cursor.flush()
            }
        }

        impl MessagePacker for CursorPacker<$b> {
            fn pack<P>(&mut self, package: P) -> io::Result<usize>
            where
                P: Packable,
            {
                package.pack(self)
            }
        }
    };
}

// Implemented this way due to architectural restrictions for cursor implementation in stdlib
impl_packer_writer!(Vec<u8>);
impl_packer_writer!(&mut [u8]);
impl_packer_writer!(&mut Vec<u8>);

impl<const N: usize> Write for CursorPacker<[u8; N]> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.cursor.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.cursor.flush()
    }
}

impl<B> BufRead for CursorPacker<B>
where
    B: AsRef<[u8]>,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.cursor.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.cursor.consume(amt)
    }
}

impl<B> Seek for CursorPacker<B>
where
    B: AsRef<[u8]>,
{
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cursor.seek(pos)
    }
}
