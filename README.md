# ulid-rs

![Continuous Integration](https://github.com/suyash/ulid-rs/workflows/Continuous%20Integration/badge.svg)

Rewrites https://github.com/suyash/ulid from C++ to Rust

This exposes a single interface for Ulid creation

```rust
Ulid::new(u64, Fn() -> u8)
```

Takes the last 48 bits of the passed timestamp and calls the passed closure
10 times for a random value.

In place of explicit MarshalBinary and UnmarshalBinary, implements
`Into<[u8; 16]>`, `Into<&[u8]>`, `Into<Vec<u8>>`, `From<[u8; 16]>` and `TryFrom<&[u8]>`

Along with `marshal` that returns 26 UTF-8 words, `TryInto<String>`, `TryInto<&str>`
and `ToString` are also implemented.

Along with `unmarshal` that works with `AsRef<[u8]>`, `TryFrom<String>` and `TryFrom<&str>`
are also implemented.

Most benchmarks line up with similar performance from C++, with some showing
improvements. Benchmarks are run on GitHub actions using criterion.

## Benchmarks

C++ results: https://github.com/suyash/ulid#benchmarks

## License

ulid-rs is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
