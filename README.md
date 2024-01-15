# Rust Minesweeper <img src="icons\Bomb-Icon-256.png" height="32" width="32"/>

Minesweeper in Rust and Bevy

Based on the Minesweeper tutorial series [here](https://dev.to/qongzi/series/16975) and adapted to the latest Bevy version (`0.12`)

## Features

TODO: Fill this list

* Rust Minesweeper clone adapted from tutorial [here](https://dev.to/qongzi/series/16975) to Bevy 0.12
* Assets and icons created using `Aseprite` (https://github.com/aseprite/aseprite). To build from source see this guide: https://gist.github.com/luciopaiva/6a1f870f932a5f54011cc869c4d558a8

## Learning resources

* https://dev.to/qongzi/series/16975
* https://github.com/leonidv/bevy-minesweeper-tutorial
* https://bevyengine.org/learn/book/getting-started/
* https://bevy-cheatbook.github.io/
* https://nnethercote.github.io/perf-book/introduction.html

## Running the debug build

You can run the debug build using `debug` feature flag with:
```
git clone https://github.com/jpiechowka/rust-minesweeper.git
cd rust-minesweeper
cargo run --package rust-minesweeper --bin rust-minesweeper --features debug
```

Logging can be configured using the `RUST_LOG` environment variable (https://bevy-cheatbook.github.io/fundamentals/log.html#environment-variable)

## Building

Install Rust (https://www.rust-lang.org/tools/install), then run the commands below:

```
git clone https://github.com/jpiechowka/rust-minesweeper.git
cd rust-minesweeper
cargo build --release
```

### Using RUSTFLAGS env variable

If you do not care that much about the compatibility of your binary on older (or other types of) processors, you can
tell the compiler to generate the newest (and potentially fastest) instructions specific to a certain CPU architecture
by using `RUSTFLAGS`environment
variable (https://nnethercote.github.io/perf-book/build-configuration.html#cpu-specific-instructions)

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

The full command to clone and build will be:

```
git clone https://github.com/jpiechowka/rust-minesweeper.git
cd rust-minesweeper
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

On Windows you need to follow this pattern: https://superuser.com/a/1049433

## Gallery

TODO: Provide some pictures or video of the final game

## License

Rust ray tracer is free, open source and permissively licensed! Except where noted (below and/or in individual files),
all code in this repository is dual-licensed under either:

* MIT License (`LICENSE-MIT` file or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (`LICENSE-APACHE` file or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard
in the Rust ecosystem.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
