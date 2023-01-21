# Flash Algorithm Template

This is a flash algorithm template for writing CMSIS-Pack flash algorithms in Rust.
It can be used to generate new flash algoritms for usage with `probe-rs`.

[![Actions Status](https://img.shields.io/github/actions/workflow/status/probe-rs/flash-algorithm-template/ci.yml?branch=master)](https://github.com/probe-rs/flash-algorithm-template/actions) [![chat](https://img.shields.io/badge/chat-probe--rs%3Amatrix.org-brightgreen)](https://matrix.to/#/#probe-rs:matrix.org)

## Dependencies

Run the following requirements:

```bash
cargo install cargo-generate && cargo-binutils && rustup component add llvm-tools-preview
```

## Instantiating the template

Run

```bash
cargo generate gh:probe-rs/flash-algorithm-template
```

to generate a new project from the template.

## Building the algorithm

Building requires nightly Rust.

Just run `cargo export`. It spits out the flash algo in the probe-rs YAML format:

    flash-algorithm$ cargo export
    flash-algorithm:
      name: {{project-name}}
      instructions: sLUUIACIGUoBRguI...wRwAgcEc=
      pc_init: 0x00000000
      pc_uninit: 0x0000007c
      pc_program_page: 0x00000088
      pc_erase_sector: 0x00000084
      pc_erase_all: 0x00000080

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
