# `gratte` (a `strum` fork)

[![CI](https://github.com/clechasseur/gratte/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/clechasseur/gratte/actions/workflows/ci.yml) [![Security audit](https://github.com/clechasseur/gratte/actions/workflows/audit-check.yml/badge.svg?branch=main)](https://github.com/clechasseur/gratte/actions/workflows/audit-check.yml) [![crates.io](https://img.shields.io/crates/v/gratte.svg)](https://crates.io/crates/gratte) [![downloads](https://img.shields.io/crates/d/gratte.svg)](https://crates.io/crates/gratte) [![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/gratte) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

`gratte` is a fork of [`strum`](https://github.com/Peternator7/strum).
It defines a set of macros and traits for working with enums and strings easier in Rust.

## Installing

Add `gratte` to your dependencies:

```toml
[dependencies]
gratte = "1.0.0"
```

or by running:

```bash
cargo add gratte
```

## Usage

`gratte` has the following `derive` macros:

| Macro               | Description                                                                                              |
|---------------------|----------------------------------------------------------------------------------------------------------|
| [EnumString]        | Converts strings to enum variants based on their name.                                                   |
| [Display]           | Converts enum variants to strings                                                                        |
| [FromRepr]          | Convert from an integer to an enum.                                                                      |
| [AsRefStr]          | Implement `AsRef<str>` for `MyEnum`                                                                      |
| [IntoStaticStr]     | Implements `From<MyEnum> for &'static str` on an enum                                                    |
| [EnumIter]          | Creates a new type that iterates the variants of an enum.                                                |
| [EnumProperty]      | Add custom properties to enum variants.                                                                  |
| [EnumMessage]       | Add a verbose message to enum variants.                                                                  |
| [EnumDiscriminants] | Generate a new type with only the discriminant names.                                                    |
| [EnumCount]         | Add a constant `usize` equal to the number of variants.                                                  |
| [VariantArray]      | Adds an associated `VARIANTS` constant which is an array of all enum discriminants                       |
| [VariantNames]      | Adds an associated `VARIANTS` constant which is an array of discriminant names                           |
| [EnumTable]         | *Experimental*, creates a new type that stores an item of a specified type for each variant of the enum. |

## Debugging

To see the generated code, set the `STRUM_DEBUG` environment variable before compiling your code.
`STRUM_DEBUG=1` will dump all the generated code for every type.
`STRUM_DEBUG=YourType` will only dump the code generated on a type named `YourType`.

## Differences from `strum`

* [EnumDiscriminants] now supports custom attributes on the discriminants enum in two new formats:
  * Path only (ex: `#[strum_discriminants(non_exhaustive)]`)
  * Name/value (ex: `#[strum_discriminants(doc = "foo")]`)

## Questions? Comments?

For instructions on filing bug reports or feature requests and contributing to the project, see [CONTRIBUTING](./CONTRIBUTING.md).

## Minimum Rust version

`gratte` currently builds on Rust 1.66.1 or newer.

## `gratte`?

_gratte_ is the French translation of [_strum_](https://en.wikipedia.org/wiki/Strum).

(For more information on the original name, see [the project page](https://github.com/Peternator7/strum?tab=readme-ov-file#name). 🙂)

[EnumString]: https://docs.rs/gratte/latest/gratte/derive.EnumString.html
[Display]: https://docs.rs/gratte/latest/gratte/derive.Display.html
[AsRefStr]: https://docs.rs/gratte/latest/gratte/derive.AsRefStr.html
[IntoStaticStr]: https://docs.rs/gratte/latest/gratte/derive.IntoStaticStr.html
[EnumIter]: https://docs.rs/gratte/latest/gratte/derive.EnumIter.html
[EnumIs]: https://docs.rs/gratte/latest/gratte/derive.EnumIs.html
[EnumProperty]: https://docs.rs/gratte/latest/gratte/derive.EnumProperty.html
[EnumMessage]: https://docs.rs/gratte/latest/gratte/derive.EnumMessage.html
[EnumDiscriminants]: https://docs.rs/gratte/latest/gratte/derive.EnumDiscriminants.html
[EnumCount]: https://docs.rs/gratte/latest/gratte/derive.EnumCount.html
[FromRepr]: https://docs.rs/gratte/latest/gratte/derive.FromRepr.html
[VariantArray]: https://docs.rs/gratte/latest/gratte/derive.VariantArray.html
[VariantNames]: https://docs.rs/gratte/latest/gratte/derive.VariantNames.html
[EnumTable]: https://docs.rs/gratte/latest/gratte/derive.EnumTable.html
