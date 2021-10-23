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

// The consumer need to guarantee himself the cursor source will live long enough to satify the
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
