# music-math-rs

[![CI](https://github.com/madskjeldgaard/music-math-rs/workflows/CI/badge.svg)](https://github.com/madskjeldgaard/music-math-rs/actions?query=workflow%3ACI)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/madskjeldgaard/music-math/blob/main/LICENSE-MIT)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

This crate contains common functions and helpers for working with music / audio / DSP in Rust.

Almost all DSP, audio or music software uses variations of some of these functions, and the goal here is to provide a common set of well-tested, benchmarked and reliable set of functions that can be re-used across different projects.

It features the following modules and functions:
- `midi`: Functions for converting between MIDI note numbers and frequencies, transposing, etc.
    - `get_midi_note_name`: Get the name of a MIDI note number.
    - `get_midinote_from_name`: Get the MIDI note number of a note name.
    - `note_name_to_octave_position`: Get the octave position of a note name, eg "D" = 2.
    - `octave_position_to_note_name`: Get the note name of an octave position, eg 2 = "D".
    - `to_frequency`: Convert a MIDI note number to a frequency.
    - `transpose`: Safely transpose a MIDI note number, making sure it is within range.
- `scaling`: Functions for scaling values between different ranges, among other things it includes conversions between db and amplitudes, linlin and linexp, etc.
    - `linlin`: Linearly scale a value from one range to another.
    - `linexp`: Linearly scale a value from one range to another, but with an exponential curve.
    - `dbamp`: Convert a decibel value to an amplitude value.
    - `ampdb`: Convert an amplitude value to a decibel value.
- `binaryops`:
    - `clip`: Clip a value between a min and max value.
    - `fold`: Fold a value between a min and max value.
    - `wrap`: Wrap a value between a min and max value.
- `interpolation`:
    - `linear`: Linear interpolation between two values.
    - `hermite`: Cubic hermite interpolation between two values.

See the documentation for more information.

# Development

Only requires `just` to bootstrap all tools and configuration.

By installing this, you will set up a pre-commit hook that will run all tests and checks before committing work, it will auto-format code and generally not allow you to commit code that isn't documented or safe, among other things.

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

To benchmark:
```bash
just benchmark
```

Before committing work:
```bash
just pre-commit
```

To see all available commands:
```bash
just list
```

## Contributing to the code

All contributions are welcome – these may be fixes to code, bug reports, improvements to documentation, etc.

Clippy will enforce most code style rules, but you can also run `cargo fmt` to format your code.

Note that if you add or change code, you should also add or change tests for that code and run the benchmarks (before and after the code change) to check for performance impacts.

## License

This project is licensed under either of:
* MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[LICENSE-MIT]: ./LICENSE-MIT
