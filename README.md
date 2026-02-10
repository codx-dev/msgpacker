# MessagePacker - a no-std msgpack implementation

[![crates.io](https://img.shields.io/crates/v/msgpacker?label=latest)](https://crates.io/crates/msgpacker)
[![Documentation](https://docs.rs/msgpacker/badge.svg)](https://docs.rs/msgpacker/)
[![License](https://img.shields.io/crates/l/msgpacker.svg)]()

The protocol specification can be found [here](https://github.com/msgpack/msgpack/blob/master/spec.md).

This crate targets simplicity and performance. No dependencies are used, just the standard Rust library.

It will implement `Packable` and `Unpackable` for Rust atomic types. The traits can also be implemented manually.

## Features

- alloc: Implements the functionality for `Vec`, `String`, and unlocks custom extensions.
- derive: Enables `MsgPacker` derive convenience macro.
- strict: Will panic if there is a protocol violation of the size of a buffer; the maximum allowed size is `u32::MAX`.
- std: Will implement the `Packable` and `Unpackable` for `std` collections.
- serde: Adds support for [serde](https://crates.io/crates/serde)

## Non-uniform collections

MessagePack is a language-agnostic format. Dynamically typed languages like Python, JavaScript, and Ruby naturally allow mixed-type collections — for instance, a Python list `[0, 1694166331209.0]` containing both an integer and a float is perfectly valid. When these values are serialized into MessagePack, the resulting byte stream encodes each element with its own type tag (`u64`, `f64`, etc.), producing an array whose elements have heterogeneous types.

Rust's type system does not directly support such collections: a `Vec<T>` requires a single concrete `T`. As noted in [#18](https://github.com/codx-dev/msgpacker/issues/18), the native `Packable`/`Unpackable` traits cannot deserialize these non-uniform arrays because they rely on a statically known element type at compile time.

The `serde` feature provides a workaround: deserialize the MessagePack bytes into `serde_json::Value`, which is a dynamically typed enum that can represent any JSON-compatible value. **This will incur performance overhead** compared to the native traits, since serde uses a visitor pattern that involves runtime type dispatch and heap allocations for every element.

```rust
use msgpacker::serde;
use serde_json::Value;

// MessagePack bytes encoding a 2-element array: [0_u64, 1694166331209.0_f64]
// This kind of payload is common when receiving data from Python, JS, or other
// dynamically typed languages that don't distinguish collection element types.
let bytes: &[u8] = &[146, 0, 203, 66, 120, 167, 66, 234, 244, 144, 0];

// Deserialize into a dynamic Value — works for any valid MessagePack payload
let value: Value = serde::from_slice(bytes).unwrap();
let items = value.as_array().unwrap();

// Each element retains its original type
assert!(items[0].is_u64());
assert!(items[1].is_f64());
```

If your use case involves only uniform collections (e.g. `Vec<u64>`), prefer the native `Packable`/`Unpackable` traits for zero-overhead deserialization.

## Example

```rust
use msgpacker::prelude::*;
use std::collections::HashMap;

// boilerplate derives - those aren't required
#[derive(Debug, PartialEq, Eq)]
// this convenience derive macro will implement `Packable` and `Unpackable`
#[derive(MsgPacker)]
pub struct City {
    name: String,

    // The traits are implemented for stdlib collections. If you have a custom map, you can use the
    // directive `#[msgpacker(map)]` so the traits will be automatically implemented through the
    // iterators of the map.
    inhabitants_per_street: HashMap<String, u64>,

    // This is also automatically implemented. The manual implementation is via `#[msgpacker(array)]`.
    zones: Vec<String>,
}

// create an instance of a city.
let city = City {
    name: "Kuala Lumpur".to_string(),
    inhabitants_per_street: HashMap::from([
        ("Street 1".to_string(), 10),
        ("Street 2".to_string(), 20),
    ]),
    zones: vec!["Zone 1".to_string(), "Zone 2".to_string()],
};

// serialize the city into bytes
let mut buf = Vec::new();
let n = city.pack(&mut buf);
println!("serialized {} bytes", n);

// deserialize the city and assert correctness
let (n, deserialized) = City::unpack(&buf).unwrap();
println!("deserialized {} bytes", n);
assert_eq!(city, deserialized);
```

## Serde

Version `0.5.0` introduces [serde](https://crates.io/crates/serde) support.

```rust
use msgpacker::serde;
use serde_json::{json, Value};

let val = serde_json::json!({"foo": "bar"});
let ser = serde::to_vec(&val);
let des: Value = serde::from_slice(&ser).unwrap();

assert_eq!(val, des);
```

While it's important to recognize that `serde`'s performance can be notably slower, this is primarily due to its implementation of a visitor pattern for type serialization, rather than solely relying on the static structure of declarations. However, `serde` is broadly used and having its support is helpful since a plethora of other libraries will be automatically supported just by having this feature enabled.

For more information, refer to `Benchmarks`.

## Benchmarks

Results obtained with `AMD EPYC 7402P 24-Core Processor`.

![Image](https://github.com/user-attachments/assets/4d695e79-59bc-40c9-9e53-5a203c703462)
![Image](https://github.com/user-attachments/assets/f6a72499-9b5c-4b47-b6ea-ec4acbfea5f3)
![Image](https://github.com/user-attachments/assets/60809961-f058-4a86-952b-b6f7d7b3c9a5)
![Image](https://github.com/user-attachments/assets/de1a2be4-50e0-4dac-94c2-e4fb2ca24e2d)
![Image](https://github.com/user-attachments/assets/f88696f0-0479-43b7-a8f1-8a8ad7dab911)
![Image](https://github.com/user-attachments/assets/98277148-e2c1-4878-abd0-6b8ab5371317)

To run the benchmarks:

```sh
cd msgpacker-bench && cargo bench
```
