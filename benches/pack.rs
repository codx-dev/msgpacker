use criterion::{black_box, criterion_group, criterion_main, Criterion};

use msgpacker::prelude::*;

pub fn pack(c: &mut Criterion) {
    let message_nil = Message::Nil;
    let message_int = Message::from(i64::MIN);

    let m = (0..100)
        .map(|i| MapEntry::new("some-key".into(), i.into()))
        .collect::<Vec<MapEntry>>();

    let message_map = Message::map(m);

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

    c.bench_function("msgpack map", |b| {
        b.iter(|| message_map.pack(black_box(&mut buffer)).unwrap())
    });

    c.bench_function("msgunpack map", |b| {
        b.iter(|| Message::unpack(black_box(&mut buffer.as_slice())).unwrap())
    });

    c.bench_function("msgunpack ref map", |b| {
        b.iter(|| unsafe { MessageRef::unpack(black_box(&mut buffer.as_slice())).unwrap() })
    });
}

criterion_group!(benches, pack);
criterion_main!(benches);
