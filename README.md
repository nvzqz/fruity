# Fruity

Rusty bindings for Apple libraries, brought to you by
[@NikolaiVazquez](https://twitter.com/NikolaiVazquez).

## Index

1. [Donate](#donate)
2. [Usage](#usage)
   1. [Feature Flags](#feature-flags)
3. [Goals](#goals)
   1. [Idiomatic Rust](#idiomatic-rust)
   2. [Zero Cost](#zero-cost)
4. [License](#license)

## Donate

If this project is useful to you, consider
[sponsoring me](https://github.com/sponsors/nvzqz) or
[donating directly](https://www.paypal.me/nvzqz)!

Doing so enables me to create high-quality open source software like this. ❤️

## Usage

This library is available [on crates.io][crate] and can be used in your project
by adding the following to your project's [`Cargo.toml`]:

```toml
[dependencies.fruity]
version = "0.1.0"
```

### Feature Flags

Each module for a library or framework has its own
[feature flag](https://doc.rust-lang.org/cargo/reference/features.html)
with the same name.

For example, this is how you enable the
[`foundation`](https://docs.rs/fruity/0.1.0/fruity/foundation/index.html)
module:

```toml
[dependencies.fruity]
version = "0.1.0"
features = ["foundation"]
```

This feature transitively enables the
[`objc`](https://docs.rs/fruity/0.1.0/fruity/objc/index.html)
feature/module.

## Goals

### Idiomatic Rust

Fruity makes interfacing with these C and Objective-C APIs feel natural in Rust.

Most of these types are classes that inherit from each other. Because Rust does
not have inheritance and instead prefers composition, this crate uses [`Deref`]
to fake inheritance.

[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html

### Zero Cost

Using Fruity to interface with Objective-C libraries should have as little
runtime cost as writing the equivalent code directly in Objective-C.

## License

This project is released under either the
[MIT License](https://github.com/nvzqz/fruity/blob/main/LICENSE-MIT) or
[Apache License (Version 2.0)](https://github.com/nvzqz/fruity/blob/main/LICENSE-APACHE),
at your choosing.

[crate]: https://crates.io/crates/fruity
[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
