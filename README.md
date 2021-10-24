# MessagePacker - some Rust in the msgpack protocol

[![crates.io](https://img.shields.io/crates/v/msgpacker?label=latest)](https://crates.io/crates/msgpacker)
[![Documentation](https://docs.rs/msgpacker/badge.svg)](https://docs.rs/msgpacker/)
[![License](https://img.shields.io/crates/l/msgpacker.svg)]()

The protocol specification can be found [here](https://github.com/msgpack/msgpack/blob/master/spec.md).

This crate targets simplicity and performance. No dependencies are used, just the standard Rust library.

We have two main structures available:

* Message - Owned parsed values
* MessageRef - Message parsed by reference and bound to the lifetime of the readers source

## Example

```rust
use msgpacker::prelude::*;
use std::io::{Cursor, Seek};

let buffer = vec![0u8; 4096];
let mut cursor = Cursor::new(buffer);

let key = Message::string("some-key");
let value = Message::integer_signed(-15);
let entry = MapEntry::new(key, value);
let message = Message::map(vec![entry]);

// Write the message to the cursor
message.pack(&mut cursor).expect("Message pack failed");

cursor.rewind().expect("Reset the cursor to the beginning");

// Read the message from the cursor
let restored = Message::unpack(&mut cursor).expect("Message unpack failed");
let value = restored
    .as_map()
    .expect("A map was originally created")
    .first()
    .expect("The map contained one entry")
    .val()
    .as_integer()
    .expect("The value was an integer")
    .as_i64()
    .expect("The value was a negative integer");

assert_eq!(value, -15);

// Alternatively, we can use the index implementation
let value = restored["some-key"]
    .as_integer()
    .expect("The value was an integer")
    .as_i64()
    .expect("The value was a negative number");

assert_eq!(value, -15);
```

## Example (by ref)

```rust
use msgpacker::prelude::*;
use std::io::{Cursor, Seek};

let mut cursor = Cursor::new(vec![0u8; 4096]);

let key = Message::String("some-key".into());
let value = Message::Integer(Integer::signed(-15));
let entry = MapEntry::new(key, value);
let message = Message::Map(vec![entry]);

// Write the message to the cursor
message.pack(&mut cursor).expect("Message pack failed");

cursor.rewind().expect("Reset the cursor to the beginning");

// The consumer need to guarantee himself the cursor source will live long enough to satisfy the
// lifetime of the message reference.
//
// If this is guaranteed, then the function is safe.
let restored = unsafe { MessageRef::unpack(&mut cursor).expect("Message unpack failed") };

// The lifetime of `MessageRef` is not bound to the `Read` implementation because the source
// might outlive it - as in this example
let _buffer = cursor.into_inner();

// `MessageRef` behaves the same as `Message`, but the runtime cost is cheaper because it will
// avoid a couple of unnecessary copies
let value = restored
    .as_map()
    .expect("A map was originally created")
    .first()
    .expect("The map contained one entry")
    .val()
    .as_integer()
    .expect("The value was an integer")
    .as_i64()
    .expect("The value was a negative integer");

assert_eq!(value, -15);

// MessageRef also implements `Index`
let value = restored["some-key"]
    .as_integer()
    .expect("The value was an integer")
    .as_i64()
    .expect("The value was a negative number");

assert_eq!(value, -15);
```

## Benchmarks

Results obtained with `Intel(R) Core(TM) i9-9900X CPU @ 3.50GHz`

```ignore,no_run
$ cargo bench
msgpack nil             time:   [4.6849 ns 4.6898 ns 4.6961 ns]
msgunpack nil           time:   [23.654 ns 23.675 ns 23.707 ns]
msgunpack ref nil       time:   [20.253 ns 20.280 ns 20.318 ns]
msgpack int             time:   [6.0831 ns 6.0921 ns 6.1045 ns]
msgunpack int           time:   [26.375 ns 26.415 ns 26.465 ns]
msgunpack ref int       time:   [25.163 ns 25.202 ns 25.258 ns]
msgpack map 1           time:   [17.411 ns 17.432 ns 17.461 ns]
msgunpack map 1         time:   [118.56 ns 118.67 ns 118.83 ns]
msgunpack ref map 1     time:   [59.850 ns 59.898 ns 59.972 ns]
msgpack map 5           time:   [73.763 ns 73.881 ns 74.049 ns]
msgunpack map 5         time:   [539.42 ns 539.91 ns 540.56 ns]
msgunpack ref map 5     time:   [161.39 ns 161.58 ns 161.86 ns]
msgpack map 10          time:   [133.03 ns 133.18 ns 133.39 ns]
msgunpack map 10        time:   [1.0574 us 1.0583 us 1.0597 us]
msgunpack ref map 10    time:   [289.05 ns 289.43 ns 289.98 ns]
msgpack map 100         time:   [1.2123 us 1.2135 us 1.2150 us]
msgunpack map 100       time:   [9.3964 us 9.4076 us 9.4214 us]
msgunpack ref map 100   time:   [2.6246 us 2.6283 us 2.6334 us]
```
