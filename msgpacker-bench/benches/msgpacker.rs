use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::collections::BTreeMap;

pub fn pack(c: &mut Criterion) {
    let mut buffer = vec![0u8; 5 * 1024 * 1024];
    let mut buffer15m = vec![0u8; 15 * 1024 * 1024];
    let mut buffer150m = vec![0u8; 150 * 1024 * 1024];

    let string_1m = vec!['a' as u8; 1 * 1024 * 1024];
    let string_1m = String::from_utf8(string_1m).unwrap();

    let string_10m = vec!['a' as u8; 10 * 1024 * 1024];
    let string_10m = String::from_utf8(string_10m).unwrap();

    let string_100m = vec!['a' as u8; 100 * 1024 * 1024];
    let string_100m = String::from_utf8(string_100m).unwrap();

    let (
        msgpacker_nil,
        msgpacker_encoded_nil,
        msgpacker_int,
        msgpacker_encoded_int,
        msgpacker_map_1,
        msgpacker_encoded_map_1,
        msgpacker_map_5,
        msgpacker_encoded_map_5,
        msgpacker_map_10,
        msgpacker_encoded_map_10,
        msgpacker_map_100,
        msgpacker_encoded_map_100,
        msgpacker_string_1m,
        msgpacker_encoded_string_1m,
        msgpacker_string_10m,
        msgpacker_encoded_string_10m,
        msgpacker_string_100m,
        msgpacker_encoded_string_100m,
    ) = {
        use msgpacker::prelude::*;

        let nil = Message::Nil;
        nil.pack(&mut buffer.as_mut_slice()).unwrap();
        let encoded_nil = buffer.clone();

        let int = Message::from(i16::MIN);
        int.pack(&mut buffer.as_mut_slice()).unwrap();
        let encoded_int = buffer.clone();

        let m = MapEntry::new("some-key".into(), 0.into());
        let map_1 = Message::map(vec![m]);
        map_1.pack(&mut buffer.as_mut_slice()).unwrap();
        let encoded_map_1 = buffer.clone();

        let m = (0..5)
            .map(|i| MapEntry::new(format!("some-{:03}", i).into(), (i as i32).into()))
            .collect::<Vec<MapEntry>>();
        let map_5 = Message::map(m);
        map_5.pack(&mut buffer.as_mut_slice()).unwrap();
        let encoded_map_5 = buffer.clone();

        let m = (0..10)
            .map(|i| MapEntry::new(format!("some-{:03}", i).into(), (i as i32).into()))
            .collect::<Vec<MapEntry>>();
        let map_10 = Message::map(m);
        map_10.pack(&mut buffer.as_mut_slice()).unwrap();
        let encoded_map_10 = buffer.clone();

        let m = (0..100)
            .map(|i| MapEntry::new(format!("some-{:03}", i).into(), (i as i32).into()))
            .collect::<Vec<MapEntry>>();
        let map_100 = Message::map(m);
        map_100.pack(&mut buffer.as_mut_slice()).unwrap();
        let encoded_map_100 = buffer.clone();

        let string_1m = Message::from(string_1m.clone());
        string_1m.pack(&mut buffer.as_mut_slice()).unwrap();
        let encoded_string_1m = buffer.clone();

        let string_10m = Message::from(string_10m.clone());
        string_10m.pack(&mut buffer15m.as_mut_slice()).unwrap();
        let encoded_string_10m = buffer15m.clone();

        let string_100m = Message::from(string_100m.clone());
        string_100m.pack(&mut buffer150m.as_mut_slice()).unwrap();
        let encoded_string_100m = buffer150m.clone();

        (
            nil,
            encoded_nil,
            int,
            encoded_int,
            map_1,
            encoded_map_1,
            map_5,
            encoded_map_5,
            map_10,
            encoded_map_10,
            map_100,
            encoded_map_100,
            string_1m,
            encoded_string_1m,
            string_10m,
            encoded_string_10m,
            string_100m,
            encoded_string_100m,
        )
    };

    let (
        rmpv_nil,
        rmpv_encoded_nil,
        rmpv_int,
        rmpv_encoded_int,
        rmpv_map_1,
        rmpv_encoded_map_1,
        rmpv_map_5,
        rmpv_encoded_map_5,
        rmpv_map_10,
        rmpv_encoded_map_10,
        rmpv_map_100,
        rmpv_encoded_map_100,
        rmpv_string_1m,
        rmpv_encoded_string_1m,
        rmpv_string_10m,
        rmpv_encoded_string_10m,
        rmpv_string_100m,
        rmpv_encoded_string_100m,
    ) = {
        use rmpv::{encode, Value};

        let nil = Value::Nil;
        encode::write_value(&mut buffer.as_mut_slice(), &nil).unwrap();
        let encoded_nil = buffer.clone();

        let int = Value::Integer(i16::MIN.into());
        encode::write_value(&mut buffer.as_mut_slice(), &int).unwrap();
        let encoded_int = buffer.clone();

        let m = (Value::String("some-key".into()), Value::Integer(0.into()));
        let map_1 = Value::Map(vec![m]);
        encode::write_value(&mut buffer.as_mut_slice(), &map_1).unwrap();
        let encoded_map_1 = buffer.clone();

        let m = (0..5)
            .map(|i| {
                (
                    Value::String(format!("some-{:03}", i).into()),
                    Value::Integer((i as i32).into()),
                )
            })
            .collect::<Vec<(Value, Value)>>();
        let map_5 = Value::Map(m);
        encode::write_value(&mut buffer.as_mut_slice(), &map_5).unwrap();
        let encoded_map_5 = buffer.clone();

        let m = (0..10)
            .map(|i| {
                (
                    Value::String(format!("some-{:03}", i).into()),
                    Value::Integer((i as i32).into()),
                )
            })
            .collect::<Vec<(Value, Value)>>();
        let map_10 = Value::Map(m);
        encode::write_value(&mut buffer.as_mut_slice(), &map_10).unwrap();
        let encoded_map_10 = buffer.clone();

        let m = (0..100)
            .map(|i| {
                (
                    Value::String(format!("some-{:03}", i).into()),
                    Value::Integer((i as i32).into()),
                )
            })
            .collect::<Vec<(Value, Value)>>();
        let map_100 = Value::Map(m);
        encode::write_value(&mut buffer.as_mut_slice(), &map_100).unwrap();
        let encoded_map_100 = buffer.clone();

        let string_1m = Value::String(string_1m.clone().into());
        encode::write_value(&mut buffer.as_mut_slice(), &string_1m).unwrap();
        let encoded_string_1m = buffer.clone();

        let string_10m = Value::String(string_10m.clone().into());
        encode::write_value(&mut buffer15m.as_mut_slice(), &string_10m).unwrap();
        let encoded_string_10m = buffer15m.clone();

        let string_100m = Value::String(string_100m.clone().into());
        encode::write_value(&mut buffer150m.as_mut_slice(), &string_100m).unwrap();
        let encoded_string_100m = buffer150m.clone();

        (
            nil,
            encoded_nil,
            int,
            encoded_int,
            map_1,
            encoded_map_1,
            map_5,
            encoded_map_5,
            map_10,
            encoded_map_10,
            map_100,
            encoded_map_100,
            string_1m,
            encoded_string_1m,
            string_10m,
            encoded_string_10m,
            string_100m,
            encoded_string_100m,
        )
    };

    let (
        rmps_nil,
        rmps_encoded_nil,
        rmps_int,
        rmps_encoded_int,
        rmps_map_1,
        rmps_encoded_map_1,
        rmps_map_5,
        rmps_encoded_map_5,
        rmps_map_10,
        rmps_encoded_map_10,
        rmps_map_100,
        rmps_encoded_map_100,
        rmps_encoded_string_1m,
        rmps_encoded_string_10m,
        rmps_encoded_string_100m,
    ) = {
        let nil = ();
        rmp_serde::encode::write(&mut buffer.as_mut_slice(), &nil).unwrap();
        let encoded_nil = buffer.clone();

        let int = i16::MIN;
        rmp_serde::encode::write(&mut buffer.as_mut_slice(), &int).unwrap();
        let encoded_int = buffer.clone();

        let map_1 = BTreeMap::from([(String::from("some-key"), 0u64); 1]);
        rmp_serde::encode::write(&mut buffer.as_mut_slice(), &map_1).unwrap();
        let encoded_map_1 = buffer.clone();

        let m = std::iter::repeat(())
            .take(5)
            .enumerate()
            .map(|(i, _)| i)
            .map(|i| (format!("some-{:03}", i), i as i32));
        let map_5 = BTreeMap::from_iter(m);
        rmp_serde::encode::write(&mut buffer.as_mut_slice(), &map_5).unwrap();
        let encoded_map_5 = buffer.clone();

        let m = std::iter::repeat(())
            .take(10)
            .enumerate()
            .map(|(i, _)| i)
            .map(|i| (format!("some-{:03}", i), i as i32));
        let map_10 = BTreeMap::from_iter(m);
        rmp_serde::encode::write(&mut buffer.as_mut_slice(), &map_10).unwrap();
        let encoded_map_10 = buffer.clone();

        let m = std::iter::repeat(())
            .take(100)
            .enumerate()
            .map(|(i, _)| i)
            .map(|i| (format!("some-{:03}", i), i as i32));
        let map_100 = BTreeMap::from_iter(m);
        rmp_serde::encode::write(&mut buffer.as_mut_slice(), &map_100).unwrap();
        let encoded_map_100 = buffer.clone();

        rmp_serde::encode::write(&mut buffer.as_mut_slice(), &string_1m).unwrap();
        let encoded_string_1m = buffer.clone();

        rmp_serde::encode::write(&mut buffer15m.as_mut_slice(), &string_10m).unwrap();
        let encoded_string_10m = buffer15m.clone();

        rmp_serde::encode::write(&mut buffer150m.as_mut_slice(), &string_100m).unwrap();
        let encoded_string_100m = buffer150m.clone();

        (
            nil,
            encoded_nil,
            int,
            encoded_int,
            map_1,
            encoded_map_1,
            map_5,
            encoded_map_5,
            map_10,
            encoded_map_10,
            map_100,
            encoded_map_100,
            encoded_string_1m,
            encoded_string_10m,
            encoded_string_100m,
        )
    };

    let mut group = c.benchmark_group("pack nil");

    group.bench_with_input("msgpacker", &msgpacker_nil, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_nil, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &rmps_nil, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("pack int");

    group.bench_with_input("msgpacker", &msgpacker_int, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_int, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &rmps_int, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("pack map[1]");

    group.bench_with_input("msgpacker", &msgpacker_map_1, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_map_1, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &rmps_map_1, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("pack map[5]");

    group.bench_with_input("msgpacker", &msgpacker_map_5, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_map_5, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &rmps_map_5, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("pack map[10]");

    group.bench_with_input("msgpacker", &msgpacker_map_10, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_map_10, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &rmps_map_10, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("pack map[100]");

    group.bench_with_input("msgpacker", &msgpacker_map_100, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_map_100, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &rmps_map_100, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("pack string[1m]");

    group.bench_with_input("msgpacker", &msgpacker_string_1m, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_string_1m, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &string_1m, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("pack string[10m]");

    group.bench_with_input("msgpacker", &msgpacker_string_10m, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer15m.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_string_10m, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer15m.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &string_10m, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer15m.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("pack string[100m]");

    group.bench_with_input("msgpacker", &msgpacker_string_100m, |b, i| {
        b.iter(|| i.pack(black_box(&mut buffer150m.as_mut_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_string_100m, |b, i| {
        b.iter(|| rmpv::encode::write_value(black_box(&mut buffer150m.as_mut_slice()), i).unwrap())
    });

    group.bench_with_input("rmps", &string_100m, |b, i| {
        b.iter(|| rmp_serde::encode::write(black_box(&mut buffer150m.as_mut_slice()), i).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("unpack nil");

    group.bench_with_input("msgpacker", &msgpacker_encoded_nil, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_nil, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_nil, |b, i| {
        b.iter(|| rmp_serde::decode::from_read::<_, ()>(black_box(&mut i.as_slice())).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("unpack int");

    group.bench_with_input("msgpacker", &msgpacker_encoded_int, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_int, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_int, |b, i| {
        b.iter(|| rmp_serde::decode::from_read::<_, i16>(black_box(&mut i.as_slice())).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("unpack map[1]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_map_1, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_map_1, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_map_1, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read::<_, BTreeMap<String, i64>>(black_box(&mut i.as_slice()))
                .unwrap()
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack map[5]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_map_5, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_map_5, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_map_5, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read::<_, BTreeMap<String, i64>>(black_box(&mut i.as_slice()))
                .unwrap()
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack map[10]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_map_10, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_map_10, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_map_10, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read::<_, BTreeMap<String, i64>>(black_box(&mut i.as_slice()))
                .unwrap()
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack map[100]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_map_100, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_map_100, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_map_100, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read::<_, BTreeMap<String, i64>>(black_box(&mut i.as_slice()))
                .unwrap()
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack string[1m]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_string_1m, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_string_1m, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_string_1m, |b, i| {
        b.iter(|| rmp_serde::decode::from_read::<_, String>(black_box(&mut i.as_slice())).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("unpack string[10m]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_string_10m, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_string_10m, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_string_10m, |b, i| {
        b.iter(|| rmp_serde::decode::from_read::<_, String>(black_box(&mut i.as_slice())).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("unpack string[100m]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_string_100m, |b, i| {
        b.iter(|| msgpacker::Message::unpack(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmpv", &rmpv_encoded_string_100m, |b, i| {
        b.iter(|| rmpv::decode::read_value(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_string_100m, |b, i| {
        b.iter(|| rmp_serde::decode::from_read::<_, String>(black_box(&mut i.as_slice())).unwrap())
    });

    group.finish();

    let mut group = c.benchmark_group("unpack ref map[1]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_map_1, |b, i| {
        b.iter(|| unsafe { msgpacker::MessageRef::unpack(black_box(&mut i.as_slice())).unwrap() })
    });

    group.bench_with_input("rmpv", &rmpv_encoded_map_1, |b, i| {
        b.iter(|| rmpv::decode::read_value_ref(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_map_1, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read_ref::<_, BTreeMap<&str, i64>>(black_box(
                &mut i.as_slice(),
            ))
            .unwrap();
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack ref map[5]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_map_5, |b, i| {
        b.iter(|| unsafe { msgpacker::MessageRef::unpack(black_box(&mut i.as_slice())).unwrap() })
    });

    group.bench_with_input("rmpv", &rmpv_encoded_map_5, |b, i| {
        b.iter(|| rmpv::decode::read_value_ref(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_map_5, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read_ref::<_, BTreeMap<&str, i64>>(black_box(
                &mut i.as_slice(),
            ))
            .unwrap();
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack ref map[10]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_map_10, |b, i| {
        b.iter(|| unsafe { msgpacker::MessageRef::unpack(black_box(&mut i.as_slice())).unwrap() })
    });

    group.bench_with_input("rmpv", &rmpv_encoded_map_10, |b, i| {
        b.iter(|| rmpv::decode::read_value_ref(black_box(&mut i.as_slice())).unwrap())
    });

    // rmps doesn't support deserialize map as ref
    group.bench_with_input("rmps", &rmps_encoded_map_10, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read_ref::<_, BTreeMap<&str, i64>>(black_box(
                &mut i.as_slice(),
            ))
            .unwrap();
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack ref map[100]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_map_100, |b, i| {
        b.iter(|| unsafe { msgpacker::MessageRef::unpack(black_box(&mut i.as_slice())).unwrap() })
    });

    group.bench_with_input("rmpv", &rmpv_encoded_map_100, |b, i| {
        b.iter(|| rmpv::decode::read_value_ref(black_box(&mut i.as_slice())).unwrap())
    });

    // rmps doesn't support deserialize map as ref
    group.bench_with_input("rmps", &rmps_encoded_map_100, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read_ref::<_, BTreeMap<&str, i64>>(black_box(
                &mut i.as_slice(),
            ))
            .unwrap();
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack ref string[1m]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_string_1m, |b, i| {
        b.iter(|| unsafe { msgpacker::MessageRef::unpack(black_box(&mut i.as_slice())).unwrap() })
    });

    group.bench_with_input("rmpv", &rmpv_encoded_string_1m, |b, i| {
        b.iter(|| rmpv::decode::read_value_ref(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_string_1m, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read_ref::<_, &str>(black_box(&mut i.as_slice())).unwrap();
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack ref string[10m]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_string_10m, |b, i| {
        b.iter(|| unsafe { msgpacker::MessageRef::unpack(black_box(&mut i.as_slice())).unwrap() })
    });

    group.bench_with_input("rmpv", &rmpv_encoded_string_10m, |b, i| {
        b.iter(|| rmpv::decode::read_value_ref(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_string_10m, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read_ref::<_, &str>(black_box(&mut i.as_slice())).unwrap();
        })
    });

    group.finish();

    let mut group = c.benchmark_group("unpack ref string[100m]");

    group.bench_with_input("msgpacker", &msgpacker_encoded_string_100m, |b, i| {
        b.iter(|| unsafe { msgpacker::MessageRef::unpack(black_box(&mut i.as_slice())).unwrap() })
    });

    group.bench_with_input("rmpv", &rmpv_encoded_string_100m, |b, i| {
        b.iter(|| rmpv::decode::read_value_ref(black_box(&mut i.as_slice())).unwrap())
    });

    group.bench_with_input("rmps", &rmps_encoded_string_100m, |b, i| {
        b.iter(|| {
            rmp_serde::decode::from_read_ref::<_, &str>(black_box(&mut i.as_slice())).unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, pack);
criterion_main!(benches);
