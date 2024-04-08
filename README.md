# music-math
[![crates.io](https://img.shields.io/crates/v/music-math.svg)](https://crates.io/crates/music_math)
[![docs.rs](https://img.shields.io/docsrs/music-math)](https://docs.rs/music_math)
[![CI](https://github.com/madskjeldgaard/music-math/workflows/CI/badge.svg)](https://github.com/madskjeldgaard/music-math/actions?query=workflow%3ACI)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/madskjeldgaard/music-math/blob/main/LICENSE-MIT)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

This crate contains common functions and helpers for working with music / audio in Rust.

Most of these are inspired by similar functions found in different computer music languages / environments like SuperCollider, PureData, MaxMSP, etc.

It features the following modules:
- `midi`: Functions for converting between MIDI note numbers and frequencies, transposing, etc.
- `scaling`: Functions for scaling values between different ranges, among other things it includes conversions between db and amplitudes, linlin and linexp, etc.
- `binaryops`: Functions for binary operations on numbers, like clip, wrap, fold, etc.

See the documentation for more information.

# Development

Only requires `just` to bootstrap all tools and configuration.
```bash
cargo install just
just init # setup repo, install hooks and all required tools
```

To run:
```bash
just run
```

To test:
```bash
just test
```

Before committing work:
```bash
just pre-commit
```

To see all available commands:
```bash
just list
```

## License

This project is licensed under either of:
* MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[LICENSE-MIT]: ./LICENSE-MIT
