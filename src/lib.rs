//! Types for manipulating numeric primitives at the bit level.
//!
//! The `quark` crate provides traits for accessing parts of numeric primitives and adds new types
//! to represent numbers using bit counts that aren't included in the standard library.
//!
//! # Bit Indexing
//!
//! Accessing a bit or range of bits in a numeric primitive can be awkward and less than readable
//! using shifts and masks:
//! 
//! ```
//! # let big: u16 = 0x35;
//! let small = big >> 2 & 0x1f;
//! # assert_eq!(small, 0xd);
//! ```
//!
//! At a glance, it's not easy to parse things like:
//!
//!  - How many bits are contributing to the resulting value and which ones are definitely zero?
//!  - Which bits in the original value are in the result?
//!
//! Using the `BitIndex` trait, the above example can be written as:
//!
//! ```
//! # use quark::BitIndex;
//! # let big: u16 = 0x35;
//! let small = big.bits(2..7);
//! # assert_eq!(small, 0xd);
//! ```
//!
//! # Bit Masks
//!
//! The `BitMask` trait allows for easily generating a bit mask using just the length and apply
//! masks:
//!
//! ```
//! # use quark::BitMask;
//! let mask = u32::mask(8);
//! assert_eq!(mask, 0xff);
//!
//! let value: u32 = 0x1234_5678;
//! assert_eq!(value.mask_to(16), 0x5678);
//! ```
//!
//! # Bit Sizes
//!
//! When implementing a trait on numeric types, sometimes the number of bits of a type will be
//! required. One way around this is adding a `bit_size()` or `bit_length()` method to the trait in
//! order to return a constant for each type. The `BitSize` trait adds a `BIT_SIZE` constant to the
//! numeric types that can be used in implementing traits without needing another method.
//!
//! # Why `quark`?
//!
//! Because our programs are primitives at the very lowest level, types like `i32`, `u8`, and
//! `usize` are like atomic pieces of data. The `quark` crate goes to the next level down, and
//! quarks are at that next level w.r.t. atoms.
//!
//! Also, I have an affinity for names with a 'Q' because my last name starts with one.

#![deny(missing_docs)]

mod bit_index;
mod bit_mask;
mod bit_size;

pub use self::bit_index::*;
pub use self::bit_mask::*;
pub use self::bit_size::*;
