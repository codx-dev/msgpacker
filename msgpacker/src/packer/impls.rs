use crate::packer::{Packable, Unpackable};
use crate::types::{Extension, ExtensionRef, Float, Integer, MapEntry, MapEntryRef};
use crate::{Message, MessageRef};

use std::io;

macro_rules! packable {
    ($t:ty) => {
        impl Packable for $t {
            fn pack<W>(&self, mut packer: W) -> io::Result<usize>
            where
                W: io::Write,
            {
                MessageRef::from(self).pack(packer.by_ref())
            }
        }

        impl Unpackable for $t {
            fn unpack<R>(mut unpacker: R) -> io::Result<Self>
            where
                R: io::BufRead,
            {
                // Safety: The temporary message reference lives long enough until deserialize
                unsafe { MessageRef::unpack(unpacker.by_ref()) }.and_then(MessageRef::try_into)
            }
        }
    };
}

macro_rules! packable_copy {
    ($t:ty) => {
        impl Packable for $t {
            fn pack<W>(&self, mut packer: W) -> io::Result<usize>
            where
                W: io::Write,
            {
                MessageRef::from(*self).pack(packer.by_ref())
            }
        }

        impl Unpackable for $t {
            fn unpack<R>(mut unpacker: R) -> io::Result<Self>
            where
                R: io::BufRead,
            {
                // Safety: The temporary message reference lives long enough until deserialize
                unsafe { MessageRef::unpack(unpacker.by_ref()) }.and_then(MessageRef::try_into)
            }
        }
    };
}

macro_rules! packable_vec {
    ($t:ty) => {
        impl Packable for Vec<$t> {
            fn pack<W>(&self, mut packer: W) -> io::Result<usize>
            where
                W: io::Write,
            {
                self.iter()
                    .try_fold(0, |s, m| Ok(s + m.pack(packer.by_ref())?))
            }
        }

        impl Unpackable for Vec<$t> {
            fn unpack<R>(mut unpacker: R) -> io::Result<Self>
            where
                R: io::BufRead,
            {
                // Safety: The temporary message reference lives long enough until deserialize
                unsafe { MessageRef::unpack(unpacker.by_ref()) }?
                    .as_array()
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "expected vector of messages")
                    })?
                    .into_iter()
                    .cloned()
                    .map(MessageRef::try_into)
                    .collect()
            }
        }
    };
}

packable_copy!(Integer);
packable_copy!(u8);
packable_copy!(u16);
packable_copy!(u32);
packable_copy!(u64);
packable_copy!(usize);
packable_copy!(i8);
packable_copy!(i16);
packable_copy!(i32);
packable_copy!(i64);
packable_copy!(isize);
packable_copy!(bool);
packable_copy!(Float);
packable_copy!(f32);
packable_copy!(f64);
packable_copy!(&str);
packable_copy!(&[u8]);
packable!(String);
packable!(Vec<u8>);
packable!(Extension);

packable_vec!(Integer);
packable_vec!(u16);
packable_vec!(u32);
packable_vec!(u64);
packable_vec!(usize);
packable_vec!(i8);
packable_vec!(i16);
packable_vec!(i32);
packable_vec!(i64);
packable_vec!(isize);
packable_vec!(bool);
packable_vec!(Float);
packable_vec!(f32);
packable_vec!(f64);
packable_vec!(String);
packable_vec!(Vec<u8>);
packable_vec!(Extension);

impl Packable for Message {
    fn pack<W>(&self, mut packer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        self.pack(packer.by_ref())
    }
}

impl Unpackable for Message {
    fn unpack<R>(mut unpacker: R) -> io::Result<Self>
    where
        R: io::BufRead,
    {
        Message::unpack(unpacker.by_ref())
    }
}

impl Packable for Vec<Message> {
    fn pack<W>(&self, mut packer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        self.iter()
            .try_fold(0, |s, m| Ok(s + m.pack(packer.by_ref())?))
    }
}

impl Unpackable for Vec<Message> {
    fn unpack<R>(mut unpacker: R) -> io::Result<Self>
    where
        R: io::BufRead,
    {
        let v = unsafe {
            MessageRef::unpack(unpacker.by_ref())?
                .as_array()
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "expected vector of messages")
                })?
                .into_iter()
                .cloned()
                .map(|m| m.into_owned())
                .collect()
        };

        Ok(v)
    }
}

impl From<Integer> for Message {
    fn from(v: Integer) -> Self {
        Self::Integer(v)
    }
}

impl From<u8> for Message {
    fn from(v: u8) -> Self {
        Self::Integer(Integer::unsigned(v))
    }
}

impl From<u16> for Message {
    fn from(v: u16) -> Self {
        Self::Integer(Integer::unsigned(v))
    }
}

impl From<u32> for Message {
    fn from(v: u32) -> Self {
        Self::Integer(Integer::unsigned(v))
    }
}

impl From<u64> for Message {
    fn from(v: u64) -> Self {
        Self::Integer(Integer::unsigned(v))
    }
}

impl From<usize> for Message {
    fn from(v: usize) -> Self {
        Self::Integer(Integer::unsigned(v as u64))
    }
}

impl From<i8> for Message {
    fn from(v: i8) -> Self {
        Self::Integer(Integer::signed(v))
    }
}

impl From<i16> for Message {
    fn from(v: i16) -> Self {
        Self::Integer(Integer::signed(v))
    }
}

impl From<i32> for Message {
    fn from(v: i32) -> Self {
        Self::Integer(Integer::signed(v))
    }
}

impl From<i64> for Message {
    fn from(v: i64) -> Self {
        Self::Integer(Integer::signed(v))
    }
}

impl From<isize> for Message {
    fn from(v: isize) -> Self {
        Self::Integer(Integer::signed(v as i64))
    }
}

impl From<bool> for Message {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl From<Float> for Message {
    fn from(v: Float) -> Self {
        Self::Float(v)
    }
}

impl From<f32> for Message {
    fn from(v: f32) -> Self {
        Self::Float(Float::f32(v))
    }
}

impl From<f64> for Message {
    fn from(v: f64) -> Self {
        Self::Float(Float::f64(v))
    }
}

impl From<&str> for Message {
    fn from(v: &str) -> Self {
        Self::String(v.to_owned())
    }
}

impl From<String> for Message {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<Vec<u8>> for Message {
    fn from(b: Vec<u8>) -> Self {
        Self::Bin(b)
    }
}

impl From<Vec<Message>> for Message {
    fn from(a: Vec<Message>) -> Self {
        Self::Array(a)
    }
}

impl From<MapEntry> for Message {
    fn from(m: MapEntry) -> Self {
        Self::Map(vec![m])
    }
}

impl From<Vec<MapEntry>> for Message {
    fn from(m: Vec<MapEntry>) -> Self {
        Self::Map(m)
    }
}

impl From<Extension> for Message {
    fn from(e: Extension) -> Self {
        Self::Extension(e)
    }
}

impl<'a> From<&'a Extension> for MessageRef<'a> {
    fn from(e: &'a Extension) -> Self {
        MessageRef::Extension(e.to_ref())
    }
}

impl<'a> From<Integer> for MessageRef<'a> {
    fn from(v: Integer) -> Self {
        Self::Integer(v)
    }
}

impl<'a> From<u8> for MessageRef<'a> {
    fn from(v: u8) -> Self {
        Self::Integer(Integer::unsigned(v))
    }
}

impl<'a> From<u16> for MessageRef<'a> {
    fn from(v: u16) -> Self {
        Self::Integer(Integer::unsigned(v))
    }
}

impl<'a> From<u32> for MessageRef<'a> {
    fn from(v: u32) -> Self {
        Self::Integer(Integer::unsigned(v))
    }
}

impl<'a> From<u64> for MessageRef<'a> {
    fn from(v: u64) -> Self {
        Self::Integer(Integer::unsigned(v))
    }
}

impl<'a> From<usize> for MessageRef<'a> {
    fn from(v: usize) -> Self {
        Self::Integer(Integer::unsigned(v as u64))
    }
}

impl<'a> From<i8> for MessageRef<'a> {
    fn from(v: i8) -> Self {
        Self::Integer(Integer::signed(v))
    }
}

impl<'a> From<i16> for MessageRef<'a> {
    fn from(v: i16) -> Self {
        Self::Integer(Integer::signed(v))
    }
}

impl<'a> From<i32> for MessageRef<'a> {
    fn from(v: i32) -> Self {
        Self::Integer(Integer::signed(v))
    }
}

impl<'a> From<i64> for MessageRef<'a> {
    fn from(v: i64) -> Self {
        Self::Integer(Integer::signed(v))
    }
}

impl<'a> From<isize> for MessageRef<'a> {
    fn from(v: isize) -> Self {
        Self::Integer(Integer::signed(v as i64))
    }
}

impl<'a> From<bool> for MessageRef<'a> {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl<'a> From<Float> for MessageRef<'a> {
    fn from(v: Float) -> Self {
        Self::Float(v)
    }
}

impl<'a> From<f32> for MessageRef<'a> {
    fn from(v: f32) -> Self {
        Self::Float(Float::f32(v))
    }
}

impl<'a> From<f64> for MessageRef<'a> {
    fn from(v: f64) -> Self {
        Self::Float(Float::f64(v))
    }
}

impl<'a> From<&'a str> for MessageRef<'a> {
    fn from(s: &'a str) -> Self {
        Self::String(s)
    }
}

impl<'a> From<&'a String> for MessageRef<'a> {
    fn from(v: &'a String) -> Self {
        Self::String(v.as_str())
    }
}

impl<'a> From<&'a [u8]> for MessageRef<'a> {
    fn from(b: &'a [u8]) -> Self {
        Self::Bin(b)
    }
}

impl<'a> From<&'a Vec<u8>> for MessageRef<'a> {
    fn from(b: &'a Vec<u8>) -> Self {
        Self::Bin(b)
    }
}

impl<'a> From<Vec<MessageRef<'a>>> for MessageRef<'a> {
    fn from(a: Vec<MessageRef<'a>>) -> Self {
        Self::Array(a)
    }
}

impl<'a> From<&'a [MessageRef<'a>]> for MessageRef<'a> {
    fn from(a: &'a [MessageRef<'a>]) -> Self {
        Self::Array(a.to_vec())
    }
}

impl<'a> From<MapEntryRef<'a>> for MessageRef<'a> {
    fn from(m: MapEntryRef<'a>) -> Self {
        Self::Map(vec![m])
    }
}

impl<'a> From<Vec<MapEntryRef<'a>>> for MessageRef<'a> {
    fn from(m: Vec<MapEntryRef<'a>>) -> Self {
        Self::Map(m)
    }
}

impl<'a> From<&'a [MapEntryRef<'a>]> for MessageRef<'a> {
    fn from(m: &'a [MapEntryRef<'a>]) -> Self {
        Self::Map(m.to_vec())
    }
}

impl<'a> From<ExtensionRef<'a>> for MessageRef<'a> {
    fn from(e: ExtensionRef<'a>) -> Self {
        Self::Extension(e)
    }
}

macro_rules! packable_array {
    ($n:expr) => {
        impl From<[u8; $n]> for Message {
            fn from(b: [u8; $n]) -> Self {
                Self::from(b.to_vec())
            }
        }

        impl<'a> From<&'a [u8; $n]> for MessageRef<'a> {
            fn from(b: &'a [u8; $n]) -> Self {
                MessageRef::from(&b[..])
            }
        }

        impl TryFrom<Message> for [u8; $n] {
            type Error = io::Error;

            fn try_from(m: Message) -> Result<[u8; $n], Self::Error> {
                m.as_bin()
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "expected fixed array message")
                    })
                    .and_then(|b| {
                        <[u8; $n]>::try_from(b).map_err(|_| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                "expected fixed array message - invalid len",
                            )
                        })
                    })
            }
        }

        impl<'a> TryFrom<MessageRef<'a>> for [u8; $n] {
            type Error = io::Error;

            fn try_from(m: MessageRef<'a>) -> Result<[u8; $n], Self::Error> {
                m.as_bin()
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "expected fixed array message")
                    })
                    .and_then(|b| {
                        <[u8; $n]>::try_from(b).map_err(|_| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                "expected fixed array message - invalid len",
                            )
                        })
                    })
            }
        }

        impl Packable for [u8; $n] {
            fn pack<W>(&self, mut packer: W) -> io::Result<usize>
            where
                W: io::Write,
            {
                MessageRef::from(&self[..]).pack(packer.by_ref())
            }
        }

        impl Unpackable for [u8; $n] {
            fn unpack<R>(mut unpacker: R) -> io::Result<Self>
            where
                R: io::BufRead,
            {
                // Safety: The temporary message reference lives long enough until deserialize
                unsafe { MessageRef::unpack(unpacker.by_ref()) }.and_then(MessageRef::try_into)
            }
        }
    };
}

packable_array!(1);
packable_array!(2);
packable_array!(3);
packable_array!(4);
packable_array!(5);
packable_array!(6);
packable_array!(7);
packable_array!(8);
packable_array!(9);
packable_array!(10);
packable_array!(11);
packable_array!(12);
packable_array!(13);
packable_array!(14);
packable_array!(15);
packable_array!(16);
packable_array!(17);
packable_array!(18);
packable_array!(19);
packable_array!(20);
packable_array!(21);
packable_array!(22);
packable_array!(23);
packable_array!(24);
packable_array!(25);
packable_array!(26);
packable_array!(27);
packable_array!(28);
packable_array!(29);
packable_array!(30);
packable_array!(31);
packable_array!(32);
packable_array!(33);
packable_array!(34);
packable_array!(35);
packable_array!(36);
packable_array!(37);
packable_array!(38);
packable_array!(39);
packable_array!(40);
packable_array!(41);
packable_array!(42);
packable_array!(43);
packable_array!(44);
packable_array!(45);
packable_array!(46);
packable_array!(47);
packable_array!(48);
packable_array!(49);
packable_array!(50);
packable_array!(51);
packable_array!(52);
packable_array!(53);
packable_array!(54);
packable_array!(55);
packable_array!(56);
packable_array!(57);
packable_array!(58);
packable_array!(59);
packable_array!(60);
packable_array!(61);
packable_array!(62);
packable_array!(63);
packable_array!(64);

impl TryFrom<Message> for Integer {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message.as_integer().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected integer message",
        ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for Integer {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message.as_integer().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected integer message",
        ))
    }
}

impl TryFrom<Message> for u8 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u8)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected byte message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for u8 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u8)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected byte message",
            ))
    }
}

impl TryFrom<Message> for u16 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u16)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected u16 message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for u16 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u16)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected u16 message",
            ))
    }
}

impl TryFrom<Message> for u32 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u32)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected u32 message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for u32 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u32)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected u32 message",
            ))
    }
}

impl TryFrom<Message> for u64 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u64)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected u64 message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for u64 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u64)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected u64 message",
            ))
    }
}

impl TryFrom<Message> for usize {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as usize)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected usize message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for usize {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as usize)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected usize message",
            ))
    }
}

impl TryFrom<Message> for i8 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i8)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected signed byte message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for i8 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i8)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected signed byte message",
            ))
    }
}

impl TryFrom<Message> for i16 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i16)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected i16 message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for i16 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i16)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected i16 message",
            ))
    }
}

impl TryFrom<Message> for i32 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i32)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected i32 message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for i32 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i32)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected i32 message",
            ))
    }
}

impl TryFrom<Message> for i64 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i64)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected i64 message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for i64 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i64)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected i64 message",
            ))
    }
}

impl TryFrom<Message> for isize {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as isize)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected isize message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for isize {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as isize)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected isize message",
            ))
    }
}

impl TryFrom<Message> for bool {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message.as_boolean().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected bool message",
        ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for bool {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message.as_boolean().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected bool message",
        ))
    }
}

impl TryFrom<Message> for Float {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message.as_float().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected float message",
        ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for Float {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message.as_float().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected float message",
        ))
    }
}

impl TryFrom<Message> for f32 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_float()
            .and_then(|f| f.as_f32())
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected f32 message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for f32 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_float()
            .and_then(|f| f.as_f32())
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected f32 message",
            ))
    }
}

impl TryFrom<Message> for f64 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_float()
            .and_then(|f| f.as_f64())
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected f64 message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for f64 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_float()
            .and_then(|f| f.as_f64())
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected f64 message",
            ))
    }
}

impl TryFrom<Message> for String {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_string()
            .map(|s| s.to_owned())
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected string message",
            ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for &'a str {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message.as_string().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected string message",
        ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for String {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_string()
            .map(|s| s.to_owned())
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected string message",
            ))
    }
}

impl TryFrom<Message> for Vec<u8> {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message.as_bin().map(|s| s.to_owned()).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected binary message",
        ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for &'a [u8] {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message.as_bin().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected binary message",
        ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for Vec<u8> {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message.as_bin().map(|s| s.to_owned()).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected binary message",
        ))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for Extension {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        match message {
            // Safety: Owned extension will be cloned
            MessageRef::Extension(x) => Ok(unsafe { x.into_owned() }),

            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected extension message",
            )),
        }
    }
}

impl FromIterator<u8> for Message {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<u8>>().into()
    }
}

impl FromIterator<Message> for Message {
    fn from_iter<I: IntoIterator<Item = Message>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<Message>>().into()
    }
}

impl FromIterator<MapEntry> for Message {
    fn from_iter<I: IntoIterator<Item = MapEntry>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<MapEntry>>().into()
    }
}

impl<'a> FromIterator<MessageRef<'a>> for MessageRef<'a> {
    fn from_iter<I: IntoIterator<Item = MessageRef<'a>>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<MessageRef<'a>>>().into()
    }
}

impl<'a> FromIterator<MapEntryRef<'a>> for MessageRef<'a> {
    fn from_iter<I: IntoIterator<Item = MapEntryRef<'a>>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<MapEntryRef<'a>>>().into()
    }
}
