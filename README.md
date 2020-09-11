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
version = "0.2.0"
```

### Feature Flags

Each module for a library or framework has its own
[feature flag](https://doc.rust-lang.org/cargo/reference/features.html)
with the same name.

For example, this is how you enable the
[`foundation`](https://docs.rs/fruity/0.2.0/fruity/foundation/index.html)
module:

```toml
[dependencies.fruity]
version = "0.2.0"
features = ["foundation"]
```

This feature transitively enables the
[`objc`](https://docs.rs/fruity/0.2.0/fruity/objc/index.html)
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
runtime cost as writing the same code directly in Objective-C.

This is true for the following:

- **Calling object methods.**

  Method dispatch is always direct and does not need the error checking overhead
  of other wrappers that use the
  [`objc::msg_send!`](https://docs.rs/objc/0.2.*/objc/macro.msg_send.html)
  macro. This also reduces the size of your program by not emitting panics that
  would otherwise never get called.

  This library is carefully written to ensure that calls to
  [`objc_msgSend`](https://developer.apple.com/documentation/objectivec/1456712-objc_msgsend)
  are always done with the correct object type, method selector, and arguments.

- **Getting a static class.**

  Getters like
  [`NSString::class`](https://docs.rs/fruity/0.2.0/fruity/foundation/struct.NSString.html#method.class)
  retrieve the class directly through its symbol. This is instantaneous,
  especially when compared to calling into the Objective-C runtime via
  [`objc_getClass`](https://developer.apple.com/documentation/objectivec/1418952-objc_getclass).

- **Creating an `NSString` from a Rust string literal.**

  The [`nsstring!`](https://docs.rs/fruity/0.2.0/fruity/macro.nsstring.html)
  macro creates an `NSString` literal (i.e. `@"string"`) at compile time. There
  is no runtime dispatch/allocation/initialization cost.

Some parts of this library still aren't zero cost. Your help would be much
appreciated here!

These are:

- **The `selector!` macro.** See
  [issue #2](https://github.com/nvzqz/fruity/issues/2)
  for details.

## License

This project is released under either the
[MIT License](https://github.com/nvzqz/fruity/blob/main/LICENSE-MIT) or
[Apache License (Version 2.0)](https://github.com/nvzqz/fruity/blob/main/LICENSE-APACHE),
at your choosing.

[crate]: https://crates.io/crates/fruity
[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
