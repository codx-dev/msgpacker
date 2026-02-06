use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use msgpacker_bench::Value;
use rand::{distributions::Standard, prelude::*};
use rmp_serde::{decode::Deserializer, encode::Serializer};
use serde::{Deserialize, Serialize};

pub fn pack(c: &mut Criterion) {
    let values: Vec<Value> = StdRng::from_seed([0xfa; 32])
        .sample_iter(&Standard)
        .take(1000)
        .collect();

    let counts = [1, 10, 100, 1000];

    // preallocate the required bytes
    let mut bufs_msgpacker = Vec::new();
    let mut bufs_msgpacker_serde = Vec::new();
    let mut bufs_rmps = Vec::new();
    for count in counts {
        let mut buf = Vec::new();
        msgpacker::pack_array(&mut buf, values.iter().take(count));
        bufs_msgpacker.push(buf);

        let mut buf = Vec::new();
        msgpacker::serde::to_buffer(&mut buf, &values[..count]);
        bufs_msgpacker_serde.push(buf);

        let mut buf = Vec::new();
        let mut serializer = Serializer::new(&mut buf);
        (&values[..count]).serialize(&mut serializer).unwrap();
        bufs_rmps.push(buf);
    }

    let mut group = c.benchmark_group("pack");

    for (i, count) in counts.iter().enumerate() {
        group.bench_with_input(
            format!("msgpacker {count}"),
            &(&values[..*count], bufs_msgpacker[i].capacity()),
            |b, (val, buf)| {
                b.iter_batched(
                    || Vec::with_capacity(*buf),
                    |mut buf| msgpacker::pack_array(black_box(&mut buf), black_box(val.iter())),
                    BatchSize::LargeInput,
                );
            },
        );

        group.bench_with_input(
            format!("msgpacker serde {count}"),
            &(&values[..*count], bufs_msgpacker_serde[i].capacity()),
            |b, (val, buf)| {
                b.iter_batched(
                    || Vec::with_capacity(*buf),
                    |mut buf| msgpacker::serde::to_buffer(black_box(&mut buf), val),
                    BatchSize::LargeInput,
                );
            },
        );

        group.bench_with_input(
            format!("rmps {count}"),
            &(&values[..*count], bufs_rmps[i].capacity()),
            |b, (val, buf)| {
                b.iter_batched(
                    || Vec::with_capacity(*buf),
                    |mut buf| {
                        black_box(val)
                            .serialize(black_box(&mut Serializer::new(&mut buf)))
                            .unwrap()
                    },
                    BatchSize::LargeInput,
                );
            },
        );
    }

    group.finish();

    let mut group = c.benchmark_group("unpack");

    for (i, count) in counts.iter().enumerate() {
        group.bench_with_input(
            format!("msgpacker {count}"),
            &bufs_msgpacker[i],
            |b, buf| {
                b.iter(|| msgpacker::unpack_array::<Value, Vec<_>>(black_box(buf)));
            },
        );

        group.bench_with_input(
            format!("msgpacker serde {count}"),
            &bufs_msgpacker_serde[i],
            |b, buf| {
                b.iter(|| msgpacker::serde::from_slice::<Vec<Value>>(black_box(buf)));
            },
        );

        group.bench_with_input(format!("rmps {count}"), &bufs_rmps[i], |b, buf| {
            b.iter(|| {
                <Vec<Value>>::deserialize(&mut Deserializer::new(black_box(&buf[..]))).unwrap()
            });
        });
    }

    group.finish();
}

criterion_group!(benches, pack);
criterion_main!(benches);
