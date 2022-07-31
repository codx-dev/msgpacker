use crate::consts::*;
use crate::packer::{Packable, Unpackable};
use crate::types::{Extension, ExtensionRef, Float, Integer, MapEntry, MapEntryRef};
use crate::{Message, MessageRef};

use std::io;
use std::marker::PhantomData;

use super::SizeableMessage;

macro_rules! packable {
    ($t:ty) => {
        impl SizeableMessage for $t {
            fn packed_len(&self) -> usize {
                MessageRef::from(self).packed_len()
            }
        }

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
        impl SizeableMessage for $t {
            fn packed_len(&self) -> usize {
                MessageRef::from(*self).packed_len()
            }
        }

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

macro_rules! packable_copy_without_sizeable {
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

macro_rules! packable_vec_copy {
    ($t:ty) => {
        impl SizeableMessage for Vec<$t> {
            fn packed_len(&self) -> usize {
                let arr = self
                    .iter()
                    .map(|i| MessageRef::from(*i))
                    .collect::<Vec<MessageRef<'_>>>();

                MessageRef::Array(arr).packed_len()
            }
        }

        impl Packable for Vec<$t> {
            fn pack<W>(&self, mut packer: W) -> io::Result<usize>
            where
                W: io::Write,
            {
                let arr = self
                    .iter()
                    .map(|i| MessageRef::from(*i))
                    .collect::<Vec<MessageRef<'_>>>();

                MessageRef::Array(arr).pack(packer.by_ref())
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

macro_rules! packable_vec {
    ($t:ty) => {
        impl SizeableMessage for Vec<$t> {
            fn packed_len(&self) -> usize {
                let arr = self
                    .iter()
                    .map(MessageRef::from)
                    .collect::<Vec<MessageRef<'_>>>();

                MessageRef::Array(arr).packed_len()
            }
        }

        impl Packable for Vec<$t> {
            fn pack<W>(&self, mut packer: W) -> io::Result<usize>
            where
                W: io::Write,
            {
                let arr = self
                    .iter()
                    .map(MessageRef::from)
                    .collect::<Vec<MessageRef<'_>>>();

                MessageRef::Array(arr).pack(packer.by_ref())
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
packable_copy!(f32);
packable_copy!(f64);
packable_copy!(&str);
packable_copy!(&[u8]);
packable_copy_without_sizeable!(Integer);
packable_copy_without_sizeable!(Float);
packable!(String);
packable!(Vec<u8>);
packable!(Extension);

packable_vec_copy!(Integer);
packable_vec_copy!(u16);
packable_vec_copy!(u32);
packable_vec_copy!(u64);
packable_vec_copy!(usize);
packable_vec_copy!(i8);
packable_vec_copy!(i16);
packable_vec_copy!(i32);
packable_vec_copy!(i64);
packable_vec_copy!(isize);
packable_vec_copy!(bool);
packable_vec_copy!(Float);
packable_vec_copy!(f32);
packable_vec_copy!(f64);
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

impl SizeableMessage for Vec<Message> {
    fn packed_len(&self) -> usize {
        let arr = self
            .iter()
            .map(Message::to_ref)
            .collect::<Vec<MessageRef<'_>>>();

        MessageRef::Array(arr).packed_len()
    }
}

impl Packable for Vec<Message> {
    fn pack<W>(&self, mut packer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let arr = self
            .iter()
            .map(Message::to_ref)
            .collect::<Vec<MessageRef<'_>>>();

        MessageRef::Array(arr).pack(packer.by_ref())
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
                .iter()
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

impl<const N: usize> From<[u8; N]> for Message {
    fn from(b: [u8; N]) -> Self {
        Self::from(b.to_vec())
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for MessageRef<'a> {
    fn from(b: &'a [u8; N]) -> Self {
        MessageRef::from(&b[..])
    }
}

impl<const N: usize> TryFrom<Message> for [u8; N] {
    type Error = io::Error;

    fn try_from(m: Message) -> Result<[u8; N], Self::Error> {
        m.as_bin()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "expected fixed array message")
            })
            .and_then(|b| {
                <[u8; N]>::try_from(b).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "expected fixed array message - invalid len",
                    )
                })
            })
    }
}

impl<'a, const N: usize> TryFrom<MessageRef<'a>> for [u8; N] {
    type Error = io::Error;

    fn try_from(m: MessageRef<'a>) -> Result<[u8; N], Self::Error> {
        m.as_bin()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "expected fixed array message")
            })
            .and_then(|b| {
                <[u8; N]>::try_from(b).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "expected fixed array message - invalid len",
                    )
                })
            })
    }
}

impl<const N: usize> SizeableMessage for [u8; N] {
    fn packed_len(&self) -> usize {
        MessageRef::from(&self[..]).packed_len()
    }
}

impl<const N: usize> Packable for [u8; N] {
    fn pack<W>(&self, mut packer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        MessageRef::from(&self[..]).pack(packer.by_ref())
    }
}

impl<const N: usize> Unpackable for [u8; N] {
    fn unpack<R>(mut unpacker: R) -> io::Result<Self>
    where
        R: io::BufRead,
    {
        // Safety: The temporary message reference lives long enough until deserialize
        unsafe { MessageRef::unpack(unpacker.by_ref()) }.and_then(MessageRef::try_into)
    }
}

impl TryFrom<Message> for Integer {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected integer message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for Integer {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected integer message"))
    }
}

impl TryFrom<Message> for u8 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u8)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected byte message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for u8 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u8)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected byte message"))
    }
}

impl TryFrom<Message> for u16 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u16)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected u16 message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for u16 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u16)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected u16 message"))
    }
}

impl TryFrom<Message> for u32 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u32)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected u32 message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for u32 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u32)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected u32 message"))
    }
}

impl TryFrom<Message> for u64 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected u64 message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for u64 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as u64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected u64 message"))
    }
}

impl TryFrom<Message> for usize {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as usize)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected usize message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for usize {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_unsigned() as usize)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected usize message"))
    }
}

impl TryFrom<Message> for i8 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i8)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "expected signed byte message")
            })
    }
}

impl<'a> TryFrom<MessageRef<'a>> for i8 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i8)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "expected signed byte message")
            })
    }
}

impl TryFrom<Message> for i16 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i16)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected i16 message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for i16 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i16)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected i16 message"))
    }
}

impl TryFrom<Message> for i32 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i32)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected i32 message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for i32 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i32)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected i32 message"))
    }
}

impl TryFrom<Message> for i64 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected i64 message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for i64 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as i64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected i64 message"))
    }
}

impl TryFrom<Message> for isize {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as isize)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected isize message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for isize {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_integer()
            .map(|i| i.as_signed() as isize)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected isize message"))
    }
}

impl TryFrom<Message> for bool {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_boolean()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected bool message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for bool {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_boolean()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected bool message"))
    }
}

impl TryFrom<Message> for Float {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_float()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected float message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for Float {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_float()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected float message"))
    }
}

impl TryFrom<Message> for f32 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_float()
            .and_then(|f| f.as_f32())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected f32 message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for f32 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_float()
            .and_then(|f| f.as_f32())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected f32 message"))
    }
}

impl TryFrom<Message> for f64 {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_float()
            .and_then(|f| f.as_f64())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected f64 message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for f64 {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_float()
            .and_then(|f| f.as_f64())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected f64 message"))
    }
}

impl TryFrom<Message> for String {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_string()
            .map(|s| s.to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected string message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for &'a str {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_string()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected string message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for String {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_string()
            .map(|s| s.to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected string message"))
    }
}

impl TryFrom<Message> for Vec<u8> {
    type Error = io::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        message
            .as_bin()
            .map(|s| s.to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected binary message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for &'a [u8] {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_bin()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected binary message"))
    }
}

impl<'a> TryFrom<MessageRef<'a>> for Vec<u8> {
    type Error = io::Error;

    fn try_from(message: MessageRef<'a>) -> Result<Self, Self::Error> {
        message
            .as_bin()
            .map(|s| s.to_owned())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected binary message"))
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

impl SizeableMessage for () {
    fn packed_len(&self) -> usize {
        0
    }
}

impl Packable for () {
    fn pack<W>(&self, _packer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        Ok(0)
    }
}

impl<T> SizeableMessage for PhantomData<T> {
    fn packed_len(&self) -> usize {
        0
    }
}

impl<T> Packable for PhantomData<T> {
    fn pack<W>(&self, _packer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        Ok(0)
    }
}

impl<T> SizeableMessage for Option<T>
where
    T: SizeableMessage,
{
    fn packed_len(&self) -> usize {
        match self {
            Some(t) => OPTION_SOME.packed_len() + t.packed_len(),
            None => OPTION_NONE.packed_len(),
        }
    }
}

impl<T> Packable for Option<T>
where
    T: Packable,
{
    fn pack<W>(&self, mut packer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        match self {
            Some(t) => Ok(OPTION_SOME.pack(packer.by_ref())? + t.pack(packer.by_ref())?),
            None => OPTION_NONE.pack(packer.by_ref()),
        }
    }
}

impl<T, E> SizeableMessage for Result<T, E>
where
    T: SizeableMessage,
    E: SizeableMessage,
{
    fn packed_len(&self) -> usize {
        match self {
            Ok(t) => RESULT_OK.packed_len() + t.packed_len(),
            Err(e) => RESULT_ERR.packed_len() + e.packed_len(),
        }
    }
}

impl<T, E> Packable for Result<T, E>
where
    T: Packable,
    E: Packable,
{
    fn pack<W>(&self, mut packer: W) -> io::Result<usize>
    where
        W: io::Write,
    {
        match self {
            Ok(t) => Ok(RESULT_OK.pack(packer.by_ref())? + t.pack(packer.by_ref())?),
            Err(e) => Ok(RESULT_ERR.pack(packer.by_ref())? + e.pack(packer.by_ref())?),
        }
    }
}

impl Unpackable for () {
    fn unpack<R>(_unpacker: R) -> io::Result<Self>
    where
        R: io::BufRead,
    {
        Ok(())
    }
}

impl<T> Unpackable for PhantomData<T> {
    fn unpack<R>(_unpacker: R) -> io::Result<Self>
    where
        R: io::BufRead,
    {
        Ok(PhantomData)
    }
}

impl<T> Unpackable for Option<T>
where
    T: Unpackable,
{
    fn unpack<R>(mut unpacker: R) -> io::Result<Self>
    where
        R: io::BufRead,
    {
        <isize as Unpackable>::unpack(unpacker.by_ref()).and_then(|r| match r {
            OPTION_SOME => Some(T::unpack(unpacker.by_ref())).transpose(),
            OPTION_NONE => Ok(None),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected option representation",
            )),
        })
    }
}

impl<T, E> Unpackable for Result<T, E>
where
    T: Unpackable,
    E: Unpackable,
{
    fn unpack<R>(mut unpacker: R) -> io::Result<Self>
    where
        R: io::BufRead,
    {
        <isize as Unpackable>::unpack(unpacker.by_ref()).and_then(|r| match r {
            RESULT_OK => T::unpack(unpacker.by_ref()).map(|t| Ok(t)),
            RESULT_ERR => E::unpack(unpacker.by_ref()).map(|e| Err(e)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected result representation",
            )),
        })
    }
}

macro_rules! tuple {
    ( $($name:ident)+) => (
        #[allow(non_snake_case)]
        impl<$($name,)+> SizeableMessage for ($($name,)+)
        where $($name: SizeableMessage,)+
        {
            fn packed_len(&self) -> usize {
                let ($(ref $name,)+) = *self;

                0 $( + $name.packed_len())+
            }
        }

        #[allow(non_snake_case)]
        impl<$($name,)+> Packable for ($($name,)+)
        where $($name: Packable,)+
        {
            fn pack<W>(&self, mut packer: W) -> io::Result<usize>
            where
                W: io::Write,
            {
                let ($(ref $name,)+) = *self;

                Ok(0 $( + $name.pack(packer.by_ref())?)+)
            }
        }

        #[allow(non_snake_case)]
        impl<$($name,)+> Unpackable for ($($name,)+)
        where $($name: Unpackable,)+
        {
            fn unpack<R>(mut unpacker: R) -> io::Result<Self>
            where
                R: io::BufRead,
            {
                Ok(($($name::unpack(unpacker.by_ref())?,)+))
            }
        }
    );
}

tuple! { A }
tuple! { A B }
tuple! { A B C }
tuple! { A B C D }
tuple! { A B C D E }
tuple! { A B C D E F }
tuple! { A B C D E F G }
tuple! { A B C D E F G H }
tuple! { A B C D E F G H I }
tuple! { A B C D E F G H I J }
tuple! { A B C D E F G H I J K }
tuple! { A B C D E F G H I J K L }
tuple! { A B C D E F G H I J K L M }
tuple! { A B C D E F G H I J K L M N }
tuple! { A B C D E F G H I J K L M N O }
tuple! { A B C D E F G H I J K L M N O P }
