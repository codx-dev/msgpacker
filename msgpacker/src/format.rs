pub struct Format {}

impl Format {
    pub const NIL: u8 = 0xc0;
    pub const TRUE: u8 = 0xc3;
    pub const FALSE: u8 = 0xc2;
    pub const POSITIVE_FIXINT: u8 = 0x7f;
    pub const UINT8: u8 = 0xcc;
    pub const UINT16: u8 = 0xcd;
    pub const UINT32: u8 = 0xce;
    pub const UINT64: u8 = 0xcf;
    pub const INT8: u8 = 0xd0;
    pub const INT16: u8 = 0xd1;
    pub const INT32: u8 = 0xd2;
    pub const INT64: u8 = 0xd3;
    pub const FLOAT32: u8 = 0xca;
    pub const FLOAT64: u8 = 0xcb;
    pub const BIN8: u8 = 0xc4;
    pub const BIN16: u8 = 0xc5;
    pub const BIN32: u8 = 0xc6;
    pub const STR8: u8 = 0xd9;
    pub const STR16: u8 = 0xda;
    pub const STR32: u8 = 0xdb;
    pub const ARRAY16: u8 = 0xdc;
    pub const ARRAY32: u8 = 0xdd;
    pub const MAP16: u8 = 0xde;
    pub const MAP32: u8 = 0xdf;
}

#[cfg(feature = "alloc")]
impl Format {
    pub const FIXEXT1: u8 = 0xd4;
    pub const FIXEXT2: u8 = 0xd5;
    pub const FIXEXT4: u8 = 0xd6;
    pub const FIXEXT8: u8 = 0xd7;
    pub const FIXEXT16: u8 = 0xd8;
    pub const EXT8: u8 = 0xc7;
    pub const EXT16: u8 = 0xc8;
    pub const EXT32: u8 = 0xc9;
}
