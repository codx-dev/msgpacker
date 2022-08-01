use msgpacker::prelude::*;

use std::io::{self, Cursor, Seek};
use std::time::Duration;

#[test]
fn pack_unpack() -> io::Result<()> {
    let buffer = vec![0u8; 4096];
    let mut cursor = Cursor::new(buffer);

    let mut cases = vec![
        Message::Nil,
        Message::Boolean(true),
        Message::Boolean(false),
        Message::Integer(Integer::signed(i64::MIN)),
        Message::Integer(Integer::signed(i32::MIN as i64 - 1)),
        Message::Integer(Integer::signed(i32::MIN as i64)),
        Message::Integer(Integer::signed(i16::MIN as i64 - 1)),
        Message::Integer(Integer::signed(i16::MIN as i64)),
        Message::Integer(Integer::signed(i8::MIN as i64 - 1)),
        Message::Integer(Integer::signed(i8::MIN as i64)),
        Message::Integer(Integer::signed(-33)),
        Message::Integer(Integer::signed(-32)),
        Message::Integer(Integer::signed(-1)),
        Message::Integer(Integer::unsigned(0u64)),
        Message::Integer(Integer::unsigned(1u64)),
        Message::Integer(Integer::unsigned(127u64)),
        Message::Integer(Integer::unsigned(128u64)),
        Message::Integer(Integer::unsigned(u8::MAX as u64)),
        Message::Integer(Integer::unsigned(u8::MAX as u64 + 1)),
        Message::Integer(Integer::unsigned(u16::MAX as u64)),
        Message::Integer(Integer::unsigned(u16::MAX as u64 + 1)),
        Message::Integer(Integer::unsigned(u32::MAX as u64)),
        Message::Integer(Integer::unsigned(u32::MAX as u64 + 1)),
        Message::Integer(Integer::unsigned(u64::MAX)),
        Message::Float(Float::F32(f32::EPSILON)),
        Message::Float(Float::F32(f32::MIN)),
        Message::Float(Float::F32(f32::MIN_POSITIVE)),
        Message::Float(Float::F32(f32::MAX)),
        Message::Float(Float::F32(f32::INFINITY)),
        Message::Float(Float::F32(f32::NEG_INFINITY)),
        Message::Float(Float::F64(f64::EPSILON)),
        Message::Float(Float::F64(f64::MIN)),
        Message::Float(Float::F64(f64::MIN_POSITIVE)),
        Message::Float(Float::F64(f64::MAX)),
        Message::Float(Float::F64(f64::INFINITY)),
        Message::Float(Float::F64(f64::NEG_INFINITY)),
        Message::String(String::from("")),
        Message::String(unsafe { String::from_utf8_unchecked(vec!['a' as u8; 31]) }),
        Message::String(unsafe { String::from_utf8_unchecked(vec!['a' as u8; 32]) }),
        Message::String(unsafe { String::from_utf8_unchecked(vec!['a' as u8; u8::MAX as usize]) }),
        Message::String(unsafe {
            String::from_utf8_unchecked(vec!['a' as u8; u8::MAX as usize + 1])
        }),
        Message::String(unsafe { String::from_utf8_unchecked(vec!['a' as u8; u16::MAX as usize]) }),
        Message::String(unsafe {
            String::from_utf8_unchecked(vec!['a' as u8; u16::MAX as usize + 1])
        }),
        Message::Bin(vec![]),
        Message::Bin(vec![0xbe; 31]),
        Message::Bin(vec![0xef; 32]),
        Message::Bin(vec![0xbe; u8::MAX as usize]),
        Message::Bin(vec![0xef; u8::MAX as usize + 1]),
        Message::Bin(vec![0xbe; u16::MAX as usize]),
        Message::Bin(vec![0xef; u16::MAX as usize + 1]),
        Message::Extension(Extension::FixExt1(-2, 1)),
        Message::Extension(Extension::FixExt2(-2, [1; 2])),
        Message::Extension(Extension::FixExt4(-2, [1; 4])),
        Message::Extension(Extension::FixExt8(-2, [1; 8])),
        Message::Extension(Extension::FixExt16(-2, [1; 16])),
        Message::Extension(Extension::Ext(-2, vec![])),
        Message::Extension(Extension::Ext(-2, vec![1; u8::MAX as usize])),
        Message::Extension(Extension::Ext(-2, vec![1; u8::MAX as usize + 1])),
        Message::Extension(Extension::Ext(-2, vec![1; u16::MAX as usize])),
        Message::Extension(Extension::Ext(-2, vec![1; u16::MAX as usize + 1])),
        Message::Extension(Extension::Timestamp(Duration::new(0, 0))),
        Message::Extension(Extension::Timestamp(Duration::new(1, 0))),
        Message::Extension(Extension::Timestamp(Duration::new(u32::MAX as u64, 0))),
        Message::Extension(Extension::Timestamp(Duration::new(u32::MAX as u64 + 1, 0))),
        Message::Extension(Extension::Timestamp(Duration::new(u32::MAX as u64 + 1, 1))),
        Message::Extension(Extension::Timestamp(Duration::new(
            u32::MAX as u64 + 1,
            (1u32 << 30) - 1,
        ))),
        Message::Extension(Extension::Timestamp(Duration::new((1u64 << 34) - 1, 10000))),
        Message::Extension(Extension::Timestamp(Duration::new(1u64 << 34, 10000))),
        Message::Extension(Extension::Timestamp(Duration::new(u64::MAX, 10000))),
    ];

    let map = cases
        .as_slice()
        .windows(2)
        .map(|w| MapEntry::new(w[0].clone(), w[1].clone()))
        .collect();

    cases.push(Message::Map(map));
    cases.push(Message::Array(cases.clone()));

    for m in &cases {
        m.pack(&mut cursor)?;
        cursor.rewind()?;

        let buf_msg = cursor.get_ref().clone();

        let m_p = Message::unpack(&mut cursor)?;
        cursor.rewind()?;

        let m_ref = m.to_ref();
        let m_o = unsafe { m_ref.clone().into_owned() };

        let buf_ref = cursor.get_ref().clone();

        m_ref.pack(&mut cursor)?;
        cursor.rewind()?;

        let m_ref_p = unsafe { MessageRef::unpack(&mut cursor)? };
        cursor.rewind()?;

        assert_eq!(buf_msg, buf_ref);
        assert_eq!(m, &m_o);
        assert_eq!(m, &m_p);
        assert_eq!(m_ref, m_ref_p);
    }

    cursor.rewind()?;

    cases
        .iter()
        .try_for_each(|m| m.pack(&mut cursor).map(|_| ()))?;

    cursor.rewind()?;

    let cases_p: Vec<Message> = (0..cases.len())
        .map(|_| Message::unpack(&mut cursor).expect("failed to unpack"))
        .collect();

    // Assert serial reconstruction
    assert_eq!(cases, cases_p);

    Ok(())
}
