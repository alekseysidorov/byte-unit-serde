# byte-unit-serde

[![tests](https://github.com/alekseysidorov/byte-unit-serde/actions/workflows/ci.yml/badge.svg)](https://github.com/alekseysidorov/byte-unit-serde/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/byte-unit-serde.svg)](https://crates.io/crates/byte-unit-serde)
[![Documentation](https://docs.rs/byte-unit-serde/badge.svg)](https://docs.rs/byte-unit-serde)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/byte-unit-serde)](./LICENSE)

<!-- ANCHOR: description -->

This crate provides de/serialization helper for [`byte-unit`] crate to use in combination with [serde's with-annotation]. 

The idea of this crate is heavily inspired by [`humantime-serde`].

## Example

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    #[serde(with = "byte_unit_serde")]
    max_size: u64,
    #[serde(default, with = "byte_unit_serde")]
    min_size: Option<u64>,
}
```

[serde's with-annotation]: https://serde.rs/field-attrs.html#with
[`humantime-serde`]: https://github.com/jean-airoldie/humantime-serde

<!-- ANCHOR_END: description -->

[`byte-unit`]: https://github.com/magiclen/byte-unit
