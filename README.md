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

## Example

```rust
use msgpacker::prelude::*;
use std::collections::HashMap;

// boilerplate derives - those aren't required
#[derive(Debug, PartialEq, Eq)]
// this convenience derive macro will implement `Packable` and `Unpackable`
#[derive(MsgPacker)]
pub struct City {
    // A `String` implements `MsgPacker` by default
    name: String,

    // Maps have a special treatment on the protocol. This directive will automatically
    // implement `MsgPacker` for any map-like type (i.e. interacts with iterators of key/value
    // pairs).
    #[msgpacker(map)]
    inhabitants_per_street: HashMap<String, u64>,

    // Arrays also have a special treatment on the protocol. This directive will automatically
    // implement `MsgPacker` for any iterator of types that implements `MsgPacker`.
    // This is not implemented by default because there is a special case for `Vec<u8>`, that
    // has a dedicated protocol binary format.
    #[msgpacker(array)]
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

## Benchmarks

Results obtained with `Intel(R) Core(TM) i9-9900X CPU @ 3.50GHz`.

The simplicity of the implementation unlocks a performance more than ~10x better than [rmp-serde](https://crates.io/crates/rmp-serde).

![image](https://github.com/codx-dev/msgpacker/assets/8730839/ef69622d-0e2f-4bb1-b47c-6412d89fc19a)
![image](https://github.com/codx-dev/msgpacker/assets/8730839/ce2de037-252a-4c90-b429-430d131ccf7e)
![image](https://github.com/codx-dev/msgpacker/assets/8730839/5576f99d-6f37-4907-89db-5d666b13f9d5)
![image](https://github.com/codx-dev/msgpacker/assets/8730839/234c31d2-f319-414b-9418-4103e97d0a9c)
