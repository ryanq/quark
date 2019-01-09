# `quark`

Types for manipulating numeric primitives at the bit level.

[![Build Status](https://travis-ci.org/ryanq/quark.svg?branch=master)](https://travis-ci.org/ryanq/quark)
[![Coverage Status](https://coveralls.io/repos/github/ryanq/quark/badge.svg?branch=master)](https://coveralls.io/github/ryanq/quark?branch=master)
[![Docs.rs](https://docs.rs/quark/badge.svg)](https://docs.rs/quark)

The `quark` crate provides traits for accessing parts of numeric primitives and
adds new types to represent numbers using bit counts that aren't included in the
standard library.

## Bit Operations

Accessing a bit or range of bits in a numeric primitive can be awkward and less
than readable using shifts and masks. Using the `BitIndex` trait, these lines
are the same:

```rust
let small = big >> 2 & 0x1f;
let small = big.bits(2..7);
```

The `BitMask` trait allows for easily generating a bit mask using just the
length and applying masks:

```rust
assert_eq!(u32::mask(8), 0xff);
assert_eq!(0x1234_5678_u32.mask_to(16), 0x5678);
```

## Sign Extension

The `Signs` trait adds methods for checking the sign bit on unsigned primitives
(and signed ones) and for sign-extending values an arbitrary number of bits:

```rust
let signed = unsigned.sign_extend(8);
```

## Why `quark`?

Because types like `i32`, `u8`, and `usize` are like atoms of data. The `quark`
crate goes one level down, and quarks are one level down from atoms.
