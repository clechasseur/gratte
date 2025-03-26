This guide is meant for people wishing to contribute to this open-source project. For more information on contributing, see [CONTRIBUTING](CONTRIBUTING.md).

## Prerequisites

### Rust

You need at least **Rust 1.66.1** to build this project's code and run the tests. You can install Rust from the [official website](https://www.rust-lang.org/tools/install).
If you already have a version of Rust installed via `rustup` but it's too old, you can update by running

```bash
rustup update
```

### Just

[just](https://github.com/casey/just) is a command-line tool to run scripts, a bit like `npm`'s scripts. It's written in Rust.

This project includes a [justfile](justfile) that makes it easier to run the various tools used for development. To install `just` via `cargo`, simply run

```bash
cargo install just
```

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), it'll probably be faster to use it instead:

```bash
cargo binstall just
```

You can also install it via various [methods](https://github.com/casey/just#packages).
## Development

### Running the tests

In order to run all tests, you can use

```bash
just test
```

Any new feature or bug fix would need new tests to validate. Make sure all tests pass before submitting a PR.

## Questions?

If any part of this documentation is unclear, please open a [new issue](https://github.com/clechasseur/gratte/issues/new/choose) so it can be fixed.
