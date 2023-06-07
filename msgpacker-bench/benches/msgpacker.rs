use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
    let mut bufs_rmps = Vec::new();
    for count in counts {
        let mut buf = Vec::new();
        msgpacker::pack_array(&mut buf, values.iter().take(count));
        bufs_msgpacker.push(buf);

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
                let mut buf = Vec::with_capacity(*buf);
                b.iter(|| msgpacker::pack_array(black_box(&mut buf), black_box(val.iter())));
            },
        );

        group.bench_with_input(
            format!("rmps {count}"),
            &(&values[..*count], bufs_rmps[i].capacity()),
            |b, (val, buf)| {
                let mut buf = Vec::with_capacity(*buf);
                let mut serializer = Serializer::new(&mut buf);
                b.iter(|| {
                    black_box(val)
                        .serialize(black_box(&mut serializer))
                        .unwrap()
                });
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

        group.bench_with_input(format!("rmps {count}"), &bufs_rmps[i], |b, buf| {
            b.iter(|| <Vec<Value>>::deserialize(&mut Deserializer::new(&buf[..])).unwrap());
        });
    }

    group.finish();
}

criterion_group!(benches, pack);
criterion_main!(benches);
