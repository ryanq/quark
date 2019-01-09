#![deny(missing_docs)]

//! Types for manipulating numeric primitives at the bit level.
//!
//! The `quark` crate provides traits for accessing parts of numeric primitives and adds new types
//! to represent numbers using bit counts that aren't included in the standard library.
//!
//! # Bit Operations
//!
//! Accessing a bit or range of bits in a numeric primitive can be awkward and less than readable
//! using shifts and masks. Using the [`BitIndex`](trait.BitIndex.html) trait, these lines are the
//! same:
//!
//! ```
//! # use quark::BitIndex;
//! # let big: u16 = 0x35;
//! let small = big >> 2 & 0x1f;
//! let small = big.bits(2..7);
//! # assert_eq!(small, 0xd);
//! ```
//!
//! The [`BitMask`](trait.BitMask.html) trait allows for easily generating a bit mask using just the
//! length and applying masks:
//!
//! ```
//! # use quark::BitMask;
//! assert_eq!(u32::mask(8), 0xff);
//! assert_eq!(0x1234_5678_u32.mask_to(16), 0x5678);
//! ```
//!
//! # Sign Extension
//!
//! The [`Signs`](trait.Signs.html) trait adds methods for checking the sign bit on unsigned
//! primitives (and signed ones) and for sign-extending values an arbitrary number of bits:
//!
//! ```
//! # use quark::Signs;
//! # let unsigned = 0x00ff_ffffu32;
//! let signed = unsigned.sign_extend(8);
//! # assert_eq!(signed, 0xffff_ffff);
//! ```
//!
//! # Why `quark`?
//!
//! Because types like `i32`, `u8`, and `usize` are like atoms of data. The `quark` crate goes one
//! level down, and quarks are one level down from atoms.

mod bit_index;
mod bit_mask;
mod bit_size;
mod signs;

pub use self::bit_index::*;
pub use self::bit_mask::*;
pub use self::bit_size::*;
pub use self::signs::*;
