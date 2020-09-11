# Fruity

Rusty bindings for Apple libraries, brought to you by
[@NikolaiVazquez](https://twitter.com/NikolaiVazquez).

## Donate

If this project is useful to you, consider
[sponsoring me](https://github.com/sponsors/nvzqz) or
[donating directly](https://www.paypal.me/nvzqz)!

Doing so enables me to create high-quality open source software like this. ❤️

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

This project is released under either:

- [MIT License](https://github.com/nvzqz/fruity/blob/main/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/nvzqz/fruity/blob/main/LICENSE-APACHE)

at your choosing.
