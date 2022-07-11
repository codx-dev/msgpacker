use std::io::{self, BufRead, Cursor, Read, Seek, SeekFrom, Write};

mod impls;

/// Define a type that can be packed into a writer.
pub trait Packable: Sized {
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
}

/// An unpacker implementation that can output messages
pub trait MessageUnpacker {
    /// Unpack a message from the internal reader
    fn unpack<P>(&mut self) -> io::Result<P>
    where
        P: Unpackable;
}

/// A packer/unpacker implementation with an underlying [`Cursor`]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CursorPacker<B> {
    cursor: Cursor<B>,
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
