# Flash Algorithm Template

This is a flash algorithm template for writing CMSIS-Pack flash algorithms in Rust.
It can be used to generate new flash algoritms for usage with `probe-rs`.

[![Actions Status](https://img.shields.io/github/actions/workflow/status/probe-rs/flash-algorithm-template/ci.yml?branch=master)](https://github.com/probe-rs/flash-algorithm-template/actions) [![chat](https://img.shields.io/badge/chat-probe--rs%3Amatrix.org-brightgreen)](https://matrix.to/#/#probe-rs:matrix.org)

## Dependencies

Run the following requirements:

```bash
cargo install cargo-generate cargo-binutils target-gen
rustup component add llvm-tools-preview
```

## Instantiating the template

Run

```bash
cargo generate gh:probe-rs/flash-algorithm-template
```

or

```bash
cargo generate gh:probe-rs/flash-algorithm-template  --name=algorithm \
-d target-arch=thumbv7em-none-eabi \
-d ram-start-address=0x20000000 \
-d ram-size=0x4000 \
-d flash-start-address=0x0 \
-d flash-size=0x40000
```

to generate a new project from the template.

## Developing the algorithm

Just run `cargo run`. It spits out the flash algo in the probe-rs YAML format and downloads it onto a target and makes a test run.
You will also be able to see RTT messages.

You can find the generated YAML in `target/definition.yaml`.

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
