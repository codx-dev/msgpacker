use criterion::{black_box, criterion_group, criterion_main, Criterion};

use msgpacker::prelude::*;

pub fn pack(c: &mut Criterion) {
    let message_nil = Message::Nil;
    let message_int = Message::from(i64::MIN);

    let m = MapEntry::new("some-key".into(), 0.into());
    let message_map_1 = Message::map(m);

    let m = (0..5)
        .map(|i| MapEntry::new("some-key".into(), i.into()))
        .collect::<Vec<MapEntry>>();

    let message_map_5 = Message::map(m);

    let m = (0..10)
        .map(|i| MapEntry::new("some-key".into(), i.into()))
        .collect::<Vec<MapEntry>>();

    let message_map_10 = Message::map(m);

    let m = (0..100)
        .map(|i| MapEntry::new("some-key".into(), i.into()))
        .collect::<Vec<MapEntry>>();

    let message_map_100 = Message::map(m);

    let mut buffer = vec![0u8; 4096];

    c.bench_function("msgpack nil", |b| {
        b.iter(|| message_nil.pack(black_box(&mut buffer)).unwrap())
    });

    c.bench_function("msgunpack nil", |b| {
        b.iter(|| Message::unpack(black_box(&mut buffer.as_slice())).unwrap())
    });

    c.bench_function("msgunpack ref nil", |b| {
        b.iter(|| unsafe { MessageRef::unpack(black_box(&mut buffer.as_slice())).unwrap() })
    });

    c.bench_function("msgpack int", |b| {
        b.iter(|| message_int.pack(black_box(&mut buffer)).unwrap())
    });

    c.bench_function("msgunpack int", |b| {
        b.iter(|| Message::unpack(black_box(&mut buffer.as_slice())).unwrap())
    });

    c.bench_function("msgunpack ref int", |b| {
        b.iter(|| unsafe { MessageRef::unpack(black_box(&mut buffer.as_slice())).unwrap() })
    });

    c.bench_function("msgpack map 1", |b| {
        b.iter(|| message_map_5.pack(black_box(&mut buffer)).unwrap())
    });

    c.bench_function("msgunpack map 1", |b| {
        b.iter(|| Message::unpack(black_box(&mut buffer.as_slice())).unwrap())
    });

    c.bench_function("msgunpack ref map 1", |b| {
        b.iter(|| unsafe { MessageRef::unpack(black_box(&mut buffer.as_slice())).unwrap() })
    });

    c.bench_function("msgpack map 5", |b| {
        b.iter(|| message_map_5.pack(black_box(&mut buffer)).unwrap())
    });

    c.bench_function("msgunpack map 5", |b| {
        b.iter(|| Message::unpack(black_box(&mut buffer.as_slice())).unwrap())
    });

    c.bench_function("msgunpack ref map 5", |b| {
        b.iter(|| unsafe { MessageRef::unpack(black_box(&mut buffer.as_slice())).unwrap() })
    });

    c.bench_function("msgpack map 10", |b| {
        b.iter(|| message_map_10.pack(black_box(&mut buffer)).unwrap())
    });

    c.bench_function("msgunpack map 10", |b| {
        b.iter(|| Message::unpack(black_box(&mut buffer.as_slice())).unwrap())
    });

    c.bench_function("msgunpack ref map 10", |b| {
        b.iter(|| unsafe { MessageRef::unpack(black_box(&mut buffer.as_slice())).unwrap() })
    });

    c.bench_function("msgpack map 100", |b| {
        b.iter(|| message_map_100.pack(black_box(&mut buffer)).unwrap())
    });

    c.bench_function("msgunpack map 100", |b| {
        b.iter(|| Message::unpack(black_box(&mut buffer.as_slice())).unwrap())
    });

    c.bench_function("msgunpack ref map 100", |b| {
        b.iter(|| unsafe { MessageRef::unpack(black_box(&mut buffer.as_slice())).unwrap() })
    });
}

criterion_group!(benches, pack);
criterion_main!(benches);
