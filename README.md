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

Benchmark results from stable (https://github.com/suyash/ulid-rs/runs/858815380)

```
Benchmarking new
Benchmarking new: Warming up for 3.0000 s
Benchmarking new: Collecting 100 samples in estimated 5.0000 s (426M iterations)
Benchmarking new: Analyzing
new                     time:   [11.716 ns 11.719 ns 11.722 ns]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low severe
  4 (4.00%) high mild
  4 (4.00%) high severe

Benchmarking new_systemtime_now
Benchmarking new_systemtime_now: Warming up for 3.0000 s
Benchmarking new_systemtime_now: Collecting 100 samples in estimated 5.0002 s (97M iterations)
Benchmarking new_systemtime_now: Analyzing
new_systemtime_now      time:   [51.380 ns 51.389 ns 51.400 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high severe

Benchmarking new_utc_now
Benchmarking new_utc_now: Warming up for 3.0000 s
Benchmarking new_utc_now: Collecting 100 samples in estimated 5.0001 s (85M iterations)
Benchmarking new_utc_now: Analyzing
new_utc_now             time:   [58.817 ns 58.842 ns 58.867 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking new_rand_random
Benchmarking new_rand_random: Warming up for 3.0000 s
Benchmarking new_rand_random: Collecting 100 samples in estimated 5.0002 s (91M iterations)
Benchmarking new_rand_random: Analyzing
new_rand_random         time:   [54.703 ns 54.708 ns 54.713 ns]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking new_systemtime_now_rand_random
Benchmarking new_systemtime_now_rand_random: Warming up for 3.0000 s
Benchmarking new_systemtime_now_rand_random: Collecting 100 samples in estimated 5.0003 s (56M iterations)
Benchmarking new_systemtime_now_rand_random: Analyzing
new_systemtime_now_rand_random
                        time:   [89.519 ns 89.544 ns 89.579 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

Benchmarking new_utc_now_rand_random
Benchmarking new_utc_now_rand_random: Warming up for 3.0000 s
Benchmarking new_utc_now_rand_random: Collecting 100 samples in estimated 5.0002 s (50M iterations)
Benchmarking new_utc_now_rand_random: Analyzing
new_utc_now_rand_random time:   [99.193 ns 99.205 ns 99.219 ns]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking marshal
Benchmarking marshal: Warming up for 3.0000 s
Benchmarking marshal: Collecting 100 samples in estimated 5.0001 s (237M iterations)
Benchmarking marshal: Analyzing
marshal                 time:   [22.176 ns 22.179 ns 22.183 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking marshal_to_string
Benchmarking marshal_to_string: Warming up for 3.0000 s
Benchmarking marshal_to_string: Collecting 100 samples in estimated 5.0002 s (108M iterations)
Benchmarking marshal_to_string: Analyzing
marshal_to_string       time:   [46.545 ns 46.575 ns 46.612 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking unmarshal
Benchmarking unmarshal: Warming up for 3.0000 s
Benchmarking unmarshal: Collecting 100 samples in estimated 5.0000 s (65M iterations)
Benchmarking unmarshal: Analyzing
unmarshal               time:   [77.037 ns 77.123 ns 77.214 ns]

Benchmarking timestamp
Benchmarking timestamp: Warming up for 3.0000 s
Benchmarking timestamp: Collecting 100 samples in estimated 5.0000 s (2.5B iterations)
Benchmarking timestamp: Analyzing
timestamp               time:   [2.0092 ns 2.0094 ns 2.0097 ns]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe
```

## License

ulid-rs is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
