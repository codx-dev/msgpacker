/// [specs](https://github.com/msgpack/msgpack/blob/master/spec.md#formats)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageFormat {
    /// 7-bits positive integer
    PositiveFixint(u8),
    /// Embeded length map
    FixMap(usize),
    /// Embeded length array
    FixArray(usize),
    /// Embeded length str
    FixStr(usize),
    /// Null representation
    Nil,
    /// Reserved by the protocol - throws error if incide
    Reserved,
    /// Boolean false representation
    False,
    /// Boolean true representation
    True,
    /// Binary data with length represented as 1 byte
    Bin8,
    /// Binary data with length represented as 2 bytes
    Bin16,
    /// Binary data with length represented as 4 bytes
    Bin32,
    /// Custom extension with length represented as 1 byte
    Ext8,
    /// Custom extension with length represented as 2 bytes
    Ext16,
    /// Custom extension with length represented as 4 bytes
    Ext32,
    /// 32-bits float representation
    Float32,
    /// 64-bits float representation
    Float64,
    /// 8-bits unsigned integer
    Uint8,
    /// 16-bits unsigned integer
    Uint16,
    /// 32-bits unsigned integer
    Uint32,
    /// 64-bits unsigned integer
    Uint64,
    /// 8-bits signed integer
    Int8,
    /// 16-bits signed integer
    Int16,
    /// 32-bits signed integer
    Int32,
    /// 64-bits signed integer
    Int64,
    /// Custom extension with 1 byte
    FixExt1,
    /// Custom extension with 2 bytes
    FixExt2,
    /// Custom extension with 4 bytes
    FixExt4,
    /// Custom extension with 8 bytes
    FixExt8,
    /// Custom extension with 16 bytes
    FixExt16,
    /// String with length represented as 1 byte
    Str8,
    /// String with length represented as 2 bytes
    Str16,
    /// String with length represented as 4 bytes
    Str32,
    /// Array of messages with length represented as 2 bytes
    Array16,
    /// Array of messages with length represented as 4 bytes
    Array32,
    /// Map of key/values with length represented as 2 bytes
    Map16,
    /// Map of key/values with length represented as 4 bytes
    Map32,
    /// 5-bits negative integer
    NegativeFixInt(i8),
}

impl From<u8> for MessageFormat {
    fn from(b: u8) -> Self {
        use MessageFormat::*;

        match b {
            0x00..=0x7f => PositiveFixint(b & 0x7f),
            0x80..=0x8f => FixMap((b & 0x0f) as usize),
            0x90..=0x9f => FixArray((b & 0x0f) as usize),
            0xa0..=0xbf => FixStr((b & 0x1f) as usize),
            0xc0 => Nil,
            0xc1 => Reserved,
            0xc2 => False,
            0xc3 => True,
            0xc4 => Bin8,
            0xc5 => Bin16,
            0xc6 => Bin32,
            0xc7 => Ext8,
            0xc8 => Ext16,
            0xc9 => Ext32,
            0xca => Float32,
            0xcb => Float64,
            0xcc => Uint8,
            0xcd => Uint16,
            0xce => Uint32,
            0xcf => Uint64,
            0xd0 => Int8,
            0xd1 => Int16,
            0xd2 => Int32,
            0xd3 => Int64,
            0xd4 => FixExt1,
            0xd5 => FixExt2,
            0xd6 => FixExt4,
            0xd7 => FixExt8,
            0xd8 => FixExt16,
            0xd9 => Str8,
            0xda => Str16,
            0xdb => Str32,
            0xdc => Array16,
            0xdd => Array32,
            0xde => Map16,
            0xdf => Map32,
            0xe0..=0xff => NegativeFixInt(b as i8),
        }
    }
}

impl From<MessageFormat> for u8 {
    fn from(m: MessageFormat) -> u8 {
        use MessageFormat::*;

        match m {
            PositiveFixint(b) => b & 0x7f,
            FixMap(b) => ((b & 0x0f) as u8) | 0x80,
            FixArray(b) => ((b & 0x0f) as u8) | 0x90,
            FixStr(b) => ((b & 0x1f) as u8) | 0xa0,
            Nil => 0xc0,
            Reserved => 0xc1,
            False => 0xc2,
            True => 0xc3,
            Bin8 => 0xc4,
            Bin16 => 0xc5,
            Bin32 => 0xc6,
            Ext8 => 0xc7,
            Ext16 => 0xc8,
            Ext32 => 0xc9,
            Float32 => 0xca,
            Float64 => 0xcb,
            Uint8 => 0xcc,
            Uint16 => 0xcd,
            Uint32 => 0xce,
            Uint64 => 0xcf,
            Int8 => 0xd0,
            Int16 => 0xd1,
            Int32 => 0xd2,
            Int64 => 0xd3,
            FixExt1 => 0xd4,
            FixExt2 => 0xd5,
            FixExt4 => 0xd6,
            FixExt8 => 0xd7,
            FixExt16 => 0xd8,
            Str8 => 0xd9,
            Str16 => 0xda,
            Str32 => 0xdb,
            Array16 => 0xdc,
            Array32 => 0xdd,
            Map16 => 0xde,
            Map32 => 0xdf,
            NegativeFixInt(b) => (b | -32i8) as u8,
        }
    }
}

#[test]
fn encode_decode() {
    for b in 0..u8::MAX {
        let f = MessageFormat::from(b);
        let f = u8::from(f);

        assert_eq!(b, f);
    }

    // NegativeFixInt supports [-32..-1]
    for i in -32i8..0 {
        let m = MessageFormat::NegativeFixInt(i);
        let n = u8::from(m);
        let n = MessageFormat::from(n);

        assert_eq!(m, n);
    }

    // PositiveFixint supports [0..127]
    for i in 0..128 {
        let m = MessageFormat::PositiveFixint(i);
        let n = u8::from(m);
        let n = MessageFormat::from(n);

        assert_eq!(m, n);
    }

    // FixMap and FixArray supports 4-bit len
    for l in 0..16 {
        let m = MessageFormat::FixMap(l);
        let n = u8::from(m);
        let n = MessageFormat::from(n);

        assert_eq!(m, n);

        let m = MessageFormat::FixArray(l);
        let n = u8::from(m);
        let n = MessageFormat::from(n);

        assert_eq!(m, n);
    }

    // FixStr supports 5-bit len
    for l in 0..32 {
        let m = MessageFormat::FixStr(l);
        let n = u8::from(m);
        let n = MessageFormat::from(n);

        assert_eq!(m, n);
    }
}
