use core::fmt;

use serde::{de, Deserializer as _};

use crate::{
    format::Format,
    unpack::{binary, collections},
    Error, Unpackable as _,
};

pub struct MsgpackDeserializer<'a>(pub &'a [u8]);

impl de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error::NotImplemented
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut MsgpackDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.0.is_empty() {
            return Err(Error::BufferTooShort);
        }
        match self.0[0] {
            0x00..=Format::POSITIVE_FIXINT => self.deserialize_u8(visitor),
            0x80..=0x8f => self.deserialize_map(visitor),
            0x90..=0x9f => self.deserialize_seq(visitor),
            0xa0..=0xbf => self.deserialize_str(visitor),
            0xe0..=0xff => self.deserialize_i8(visitor),
            Format::NIL => self.deserialize_option(visitor),
            Format::TRUE => self.deserialize_bool(visitor),
            Format::FALSE => self.deserialize_bool(visitor),
            Format::UINT8 => self.deserialize_u8(visitor),
            Format::UINT16 => self.deserialize_u16(visitor),
            Format::UINT32 => self.deserialize_u32(visitor),
            Format::UINT64 => self.deserialize_u64(visitor),
            Format::INT8 => self.deserialize_i8(visitor),
            Format::INT16 => self.deserialize_i16(visitor),
            Format::INT32 => self.deserialize_i32(visitor),
            Format::INT64 => self.deserialize_i64(visitor),
            Format::FLOAT32 => self.deserialize_f32(visitor),
            Format::FLOAT64 => self.deserialize_f64(visitor),
            Format::BIN8 | Format::BIN16 | Format::BIN32 => self.deserialize_bytes(visitor),
            Format::STR8 | Format::STR16 | Format::STR32 => self.deserialize_str(visitor),
            Format::ARRAY16 | Format::ARRAY32 => self.deserialize_seq(visitor),
            Format::MAP16 | Format::MAP32 => self.deserialize_map(visitor),
            #[cfg(feature = "alloc")]
            Format::FIXEXT1
            | Format::FIXEXT2
            | Format::FIXEXT4
            | Format::FIXEXT8
            | Format::FIXEXT16
            | Format::EXT8
            | Format::EXT16
            | Format::EXT32 => self.deserialize_byte_buf(visitor),
            _ => Err(Error::UnexpectedFormatTag),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = bool::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_bool(v)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = i8::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_i8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = i16::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = i32::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_i32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = i64::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_i64(v)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = i128::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_i128(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = u8::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_u8(v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = u16::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_u16(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = u32::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_u32(v)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = u64::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_u64(v)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = u128::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_u128(v)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = f32::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_f32(v)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = f64::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_f64(v)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = char::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_char(v)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = binary::unpack_str(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_borrowed_str(v)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        #[cfg(not(feature = "alloc"))]
        {
            let _ = visitor;
            return Err(Error::NotImplemented);
        }

        #[cfg(feature = "alloc")]
        {
            let (n, v) = ::alloc::string::String::unpack(self.0)?;
            self.0 = &self.0[n..];
            return visitor.visit_string(v);
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, v) = binary::unpack_bytes(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_borrowed_bytes(v)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        #[cfg(not(feature = "alloc"))]
        {
            let _ = visitor;
            return Err(Error::NotImplemented);
        }

        #[cfg(feature = "alloc")]
        {
            let (n, v) = ::alloc::vec::Vec::unpack(self.0)?;
            self.0 = &self.0[n..];
            return visitor.visit_byte_buf(v);
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.0.is_empty() {
            return Err(Error::BufferTooShort);
        }
        if self.0[0] == Format::NIL {
            self.0 = &self.0[1..];
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, _) = <()>::unpack(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, len) = collections::unpack_array_len(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_seq(MsgpackDeserializerSeq {
            m: self,
            count: len,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(MsgpackDeserializerSeq {
            m: self,
            count: len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(MsgpackDeserializerSeq {
            m: self,
            count: len,
        })
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (n, len) = collections::unpack_map_len(self.0)?;
        self.0 = &self.0[n..];
        visitor.visit_map(MsgpackDeserializerSeq {
            m: self,
            count: len,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(MsgpackDeserializerSeq {
            m: self,
            count: fields.len(),
        })
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(MsgpackEnumHandler { de: self })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct MsgpackDeserializerSeq<'a, 'de: 'a> {
    m: &'a mut MsgpackDeserializer<'de>,
    count: usize,
}

impl<'de, 'a> de::SeqAccess<'de> for MsgpackDeserializerSeq<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.count == 0 {
            return Ok(None);
        }
        self.count -= 1;
        seed.deserialize(&mut *self.m).map(Some)
    }
}

impl<'de, 'a> de::MapAccess<'de> for MsgpackDeserializerSeq<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.count == 0 {
            return Ok(None);
        }
        self.count -= 1;
        seed.deserialize(&mut *self.m).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.m)
    }
}

struct MsgpackEnumHandler<'a, 'de: 'a> {
    de: &'a mut MsgpackDeserializer<'de>,
}

impl<'de, 'a> de::VariantAccess<'de> for MsgpackEnumHandler<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_struct("", fields, visitor)
    }
}

impl<'de, 'a> de::EnumAccess<'de> for MsgpackEnumHandler<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let discriminant = seed.deserialize(MsgpackTagDeserializer { de: self.de })?;

        Ok((discriminant, self))
    }
}

struct MsgpackTagDeserializer<'a, 'de: 'a> {
    de: &'a mut MsgpackDeserializer<'de>,
}

impl<'de, 'a> serde::Deserializer<'de> for MsgpackTagDeserializer<'a, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_identifier(visitor)
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unreachable!()
    }
}
