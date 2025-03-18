pub struct Format {}

impl Format {
    /// Nil format stores nil in 1 byte.
    pub const NIL: u8 = 0xc0;
    /// Bool format family stores false or true in 1 byte.
    pub const TRUE: u8 = 0xc3;
    /// Bool format family stores false or true in 1 byte.
    pub const FALSE: u8 = 0xc2;
    /// Positive fixint stores 7-bit positive integer
    pub const POSITIVE_FIXINT: u8 = 0x7f;
    /// Uint 8 stores a 8-bit unsigned integer
    pub const UINT8: u8 = 0xcc;
    /// Uint 16 stores a 16-bit big-endian unsigned integer
    pub const UINT16: u8 = 0xcd;
    /// Uint 32 stores a 32-bit big-endian unsigned integer
    pub const UINT32: u8 = 0xce;
    /// Uint 64 stores a 64-bit big-endian unsigned integer
    pub const UINT64: u8 = 0xcf;
    /// Int 8 stores a 8-bit signed integer
    pub const INT8: u8 = 0xd0;
    /// Int 16 stores a 16-bit big-endian signed integer
    pub const INT16: u8 = 0xd1;
    /// Int 32 stores a 32-bit big-endian signed integer
    pub const INT32: u8 = 0xd2;
    /// Int 64 stores a 64-bit big-endian signed integer
    pub const INT64: u8 = 0xd3;
    /// Float 32 stores a floating point number in IEEE 754 single precision floating point number
    pub const FLOAT32: u8 = 0xca;
    /// Float 64 stores a floating point number in IEEE 754 double precision floating point number
    pub const FLOAT64: u8 = 0xcb;
    /// Bin 8 stores a byte array whose length is upto (2^8)-1 bytes
    pub const BIN8: u8 = 0xc4;
    /// Bin 16 stores a byte array whose length is upto (2^16)-1 bytes
    pub const BIN16: u8 = 0xc5;
    /// Bin 32 stores a byte array whose length is upto (2^32)-1 bytes
    pub const BIN32: u8 = 0xc6;
    /// Str 8 stores a byte array whose length is upto (2^8)-1 bytes
    pub const STR8: u8 = 0xd9;
    /// Str 16 stores a byte array whose length is upto (2^16)-1 bytes
    pub const STR16: u8 = 0xda;
    /// Str 32 stores a byte array whose length is upto (2^32)-1 bytes
    pub const STR32: u8 = 0xdb;
    /// Array 16 stores an array whose length is upto (2^16)-1 elements
    pub const ARRAY16: u8 = 0xdc;
    /// Array 32 stores an array whose length is upto (2^32)-1 elements
    pub const ARRAY32: u8 = 0xdd;
    /// Map 16 stores a map whose length is upto (2^16)-1 elements
    pub const MAP16: u8 = 0xde;
    /// Map 32 stores a map whose length is upto (2^32)-1 elements
    pub const MAP32: u8 = 0xdf;
}

#[cfg(feature = "alloc")]
impl Format {
    /// Fixext 1 stores an integer and a byte array whose length is 1 byte
    pub const FIXEXT1: u8 = 0xd4;
    /// Fixext 2 stores an integer and a byte array whose length is 2 byte
    pub const FIXEXT2: u8 = 0xd5;
    /// Fixext 4 stores an integer and a byte array whose length is 4 byte
    pub const FIXEXT4: u8 = 0xd6;
    /// Fixext 8 stores an integer and a byte array whose length is 8 byte
    pub const FIXEXT8: u8 = 0xd7;
    /// Fixext 16 stores an integer and a byte array whose length is 16 byte
    pub const FIXEXT16: u8 = 0xd8;
    /// Ext 8 stores an integer and a byte array whose length is upto (2^8)-1 bytes
    pub const EXT8: u8 = 0xc7;
    /// Ext 16 stores an integer and a byte array whose length is upto (2^16)-1 bytes
    pub const EXT16: u8 = 0xc8;
    /// Ext 32 stores an integer and a byte array whose length is upto (2^32)-1 bytes
    pub const EXT32: u8 = 0xc9;
}
