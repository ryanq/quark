#![allow(unused_comparisons)]

use crate::{BitMask, BitSize};
use std::ops::RangeBounds;

/// Provides bit indexing operations.
///
/// This trait defines functions for accessing single bits to determine whether they are set and for
/// accessing ranges of bits to extract the value they contain.
///
/// # Examples
///
/// ```
/// use quark::BitIndex;
///
/// let value: u32 = 0xe01a3497;
///
/// let s = value.bit(20);
/// assert!(s);
/// let rd = value.bits(16..20);
/// assert_eq!(rd, 10);
/// let rn = value.bits(12..16);
/// assert_eq!(rn, 3);
/// let rs = value.bits(8..12);
/// assert_eq!(rs, 4);
/// let rm = value.bits(0..4);
/// assert_eq!(rm, 7);
/// ```
pub trait BitIndex: BitSize + BitMask {
    /// Returns whether the specified bit is set.
    fn bit(&self, index: usize) -> bool;

    /// Returns the bits contained in the specified bit range.
    fn bits<Idx: RangeBounds<usize>>(&self, index: Idx) -> Self;
}

macro_rules! bit_index_impl {
    ($type:ty) => {
        impl BitIndex for $type {
            fn bit(&self, index: usize) -> bool {
                self.checked_shr(index as _)
                    .unwrap_or_else(|| if *self < 0 { 1 } else { 0 })
                    & 1
                    == 1
            }

            fn bits<Idx: RangeBounds<usize>>(&self, index: Idx) -> Self {
                let mask = match (index.start_bound(), index.end_bound()) {
                    (::std::ops::Bound::Excluded(se), ::std::ops::Bound::Excluded(ee)) => {
                        Some(*ee - *se - 1)
                    }
                    (::std::ops::Bound::Excluded(se), ::std::ops::Bound::Included(ee)) => {
                        Some(*ee - *se)
                    }
                    (::std::ops::Bound::Excluded(_), ::std::ops::Bound::Unbounded) => None,
                    (::std::ops::Bound::Included(si), ::std::ops::Bound::Excluded(ee)) => {
                        Some(*ee - *si)
                    }
                    (::std::ops::Bound::Included(si), ::std::ops::Bound::Included(ei)) => {
                        Some(*ei + 1 - *si)
                    }
                    (::std::ops::Bound::Included(_), ::std::ops::Bound::Unbounded) => None,
                    (::std::ops::Bound::Unbounded, ::std::ops::Bound::Excluded(ee)) => Some(*ee),
                    (::std::ops::Bound::Unbounded, ::std::ops::Bound::Included(ei)) => {
                        Some(*ei + 1)
                    }
                    (::std::ops::Bound::Unbounded, ::std::ops::Bound::Unbounded) => None,
                };

                let shift = match index.start_bound() {
                    ::std::ops::Bound::Excluded(e) => Some(*e + 1),
                    ::std::ops::Bound::Included(i) => Some(*i),
                    ::std::ops::Bound::Unbounded => Some(0),
                };

                match (shift, mask) {
                    (Some(s), Some(m)) => self
                        .checked_shr(s as _)
                        .unwrap_or_else(|| {
                            if *self < 0 {
                                (0 as Self).wrapping_sub(1)
                            } else {
                                0
                            }
                        })
                        .mask_to(m),
                    (Some(s), None) => self.checked_shr(s as _).unwrap_or_else(|| {
                        if *self < 0 {
                            (0 as Self).wrapping_sub(1)
                        } else {
                            0
                        }
                    }),
                    (None, Some(m)) => self.mask_to(m),
                    (None, None) => *self,
                }
            }
        }
    };
}

bit_index_impl!(u8);
bit_index_impl!(u16);
bit_index_impl!(u32);
bit_index_impl!(u64);
bit_index_impl!(u128);
bit_index_impl!(usize);
bit_index_impl!(i8);
bit_index_impl!(i16);
bit_index_impl!(i32);
bit_index_impl!(i64);
bit_index_impl!(i128);
bit_index_impl!(isize);

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

    struct RangeEE(usize, usize);
    impl RangeBounds<usize> for RangeEE {
        fn start_bound(&self) -> std::ops::Bound<&usize> {
            std::ops::Bound::Excluded(&self.0)
        }
        fn end_bound(&self) -> std::ops::Bound<&usize> {
            std::ops::Bound::Excluded(&self.1)
        }
    }

    struct RangeEI(usize, usize);
    impl RangeBounds<usize> for RangeEI {
        fn start_bound(&self) -> std::ops::Bound<&usize> {
            std::ops::Bound::Excluded(&self.0)
        }
        fn end_bound(&self) -> std::ops::Bound<&usize> {
            std::ops::Bound::Included(&self.1)
        }
    }

    struct RangeEU(usize);
    impl RangeBounds<usize> for RangeEU {
        fn start_bound(&self) -> std::ops::Bound<&usize> {
            std::ops::Bound::Excluded(&self.0)
        }
        fn end_bound(&self) -> std::ops::Bound<&usize> {
            std::ops::Bound::Unbounded
        }
    }

    #[test]
    fn unsigned_bit_index() {
        let byte: u8 = 90;

        asserting!("bit() looks up the correct bit")
            .that(&[byte.bit(2), byte.bit(3), byte.bit(4), byte.bit(5)])
            .is_equal_to(&[false, true, true, false]);

        asserting!("bits(RangeFull) returns the whole value")
            .that(&byte.bits(..))
            .is_equal_to(90);

        asserting!("bits(RangeTo) excludes the end bit")
            .that(&byte.bits(..4))
            .is_equal_to(10);

        asserting!("bits(RangeToInclusive) includes the end bit")
            .that(&byte.bits(..=4))
            .is_equal_to(26);

        asserting!("bits(RangeFrom) includes the start bit")
            .that(&byte.bits(4..))
            .is_equal_to(5);

        asserting!("bits(Range) includes the start bit")
            .that(&byte.bits(4..8))
            .is_equal_to(5);
        asserting!("bits(Range) excludes the end bit")
            .that(&byte.bits(0..4))
            .is_equal_to(10);

        asserting!("bits(RangeInclusive) includes the start bit")
            .that(&byte.bits(4..=7))
            .is_equal_to(5);
        asserting!("bits(RangeInclusive) includes the end bit")
            .that(&byte.bits(0..=4))
            .is_equal_to(26);

        asserting!("bits(RangeEU) excludes the start bit")
            .that(&byte.bits(RangeEU(4)))
            .is_equal_to(2);

        asserting!("bits(RangeEE) excludes the start bit")
            .that(&byte.bits(RangeEE(4, 8)))
            .is_equal_to(2);
        asserting!("bits(RangeEE) excludes the end bit")
            .that(&byte.bits(RangeEE(0, 4)))
            .is_equal_to(5);

        asserting!("bits(RangeEI) excludes the start bit")
            .that(&byte.bits(RangeEI(4, 7)))
            .is_equal_to(2);
        asserting!("bits(RangeEI) includes the end bit")
            .that(&byte.bits(RangeEI(0, 4)))
            .is_equal_to(13);
    }

    #[test]
    fn unsigned_extra_high_bits() {
        let byte: u8 = 90;

        asserting!("bit() returns 0 when indexing past the last bit")
            .that(&[byte.bit(8), byte.bit(9), byte.bit(10)])
            .is_equal_to(&[false, false, false]);

        asserting!("bits(RangeTo) can index past the last bit")
            .that(&byte.bits(..16))
            .is_equal_to(90);

        asserting!("bits(RangeToInclusive) can index past the last bit")
            .that(&byte.bits(..=15))
            .is_equal_to(90);

        asserting!("bits(RangeFrom) can index past the last bit")
            .that(&byte.bits(4..))
            .is_equal_to(5);
        asserting!("bits(RangeFrom) is 0 when completely past the last bit")
            .that(&byte.bits(8..))
            .is_equal_to(0);

        asserting!("bits(Range) can index past the last bit")
            .that(&byte.bits(4..16))
            .is_equal_to(5);
        asserting!("bits(Range) is 0 when completely past the last bit")
            .that(&byte.bits(8..16))
            .is_equal_to(0);

        asserting!("bits(RangeInclusive) can index past the last bit")
            .that(&byte.bits(4..=15))
            .is_equal_to(5);
        asserting!("bits(RangeInclusive) is 0 when completely past the last bit")
            .that(&byte.bits(8..=15))
            .is_equal_to(0);

        asserting!("bits(RangeEU) can index past the last bit")
            .that(&byte.bits(RangeEU(4)))
            .is_equal_to(2);
        asserting!("bits(RangeEU) is 0 when completely past the last bit")
            .that(&byte.bits(RangeEU(8)))
            .is_equal_to(0);

        asserting!("bits(RangeEE) can index past the last bit")
            .that(&byte.bits(RangeEE(4, 16)))
            .is_equal_to(2);
        asserting!("bits(RangeEE) is 0 when completely past the last bit")
            .that(&byte.bits(RangeEE(8, 16)))
            .is_equal_to(0);

        asserting!("bits(RangeEI) can index past the last bit")
            .that(&byte.bits(RangeEI(4, 15)))
            .is_equal_to(2);
        asserting!("bits(RangeEI) is 0 when completely past the last bit")
            .that(&byte.bits(RangeEI(8, 15)))
            .is_equal_to(0);
    }

    #[test]
    fn signed_bit_index() {
        let byte: i8 = -90;

        asserting!("bit() looks up the correct bit")
            .that(&[byte.bit(2), byte.bit(3), byte.bit(4), byte.bit(5)])
            .is_equal_to(&[true, false, false, true]);

        asserting!("bits(Range) is equal to the equivalent shift and mask")
            .that(&byte.bits(2..6))
            .is_equal_to(byte >> 2 & 0xf);
        asserting!("bits(RangeFrom) is equal to the equivalent shift and mask")
            .that(&byte.bits(2..))
            .is_equal_to(byte >> 2);

        asserting!("bits(RangeFull) returns the whole value")
            .that(&byte.bits(..))
            .is_equal_to(-90);

        asserting!("bits(RangeTo) excludes the end bit")
            .that(&byte.bits(..5))
            .is_equal_to(6);

        asserting!("bits(RangeToInclusive) includes the end bit")
            .that(&byte.bits(..=5))
            .is_equal_to(38);

        asserting!("bits(RangeFrom) includes the start bit")
            .that(&byte.bits(4..))
            .is_equal_to(-6);

        asserting!("bits(Range) includes the start bit")
            .that(&byte.bits(4..8))
            .is_equal_to(10);
        asserting!("bits(Range) excludes the end bit")
            .that(&byte.bits(0..5))
            .is_equal_to(6);

        asserting!("bits(RangeInclusive) includes the start bit")
            .that(&byte.bits(4..=7))
            .is_equal_to(10);
        asserting!("bits(RangeInclusive) includes the end bit")
            .that(&byte.bits(0..=5))
            .is_equal_to(38);

        asserting!("bits(RangeEU) excludes the start bit")
            .that(&byte.bits(RangeEU(4)))
            .is_equal_to(-3);

        asserting!("bits(RangeEE) excludes the start bit")
            .that(&byte.bits(RangeEE(4, 8)))
            .is_equal_to(5);
        asserting!("bits(RangeEE) excludes the end bit")
            .that(&byte.bits(RangeEE(0, 2)))
            .is_equal_to(1);

        asserting!("bits(RangeEI) excludes the start bit")
            .that(&byte.bits(RangeEI(2, 4)))
            .is_equal_to(0);
        asserting!("bits(RangeEI) includes the end bit")
            .that(&byte.bits(RangeEI(2, 5)))
            .is_equal_to(4);
    }

    #[test]
    fn signed_extra_high_bits() {
        let byte: i8 = -90;

        asserting!("bit() returns 1 when indexing past the last bit")
            .that(&[byte.bit(8), byte.bit(9), byte.bit(10)])
            .is_equal_to(&[true, true, true]);

        asserting!("bits(RangeTo) can index past the last bit")
            .that(&byte.bits(..16))
            .is_equal_to(-90);

        asserting!("bits(RangeToInclusive) can index past the last bit")
            .that(&byte.bits(..=15))
            .is_equal_to(-90);

        asserting!("bits(RangeFrom) can index past the last bit")
            .that(&byte.bits(4..))
            .is_equal_to(-6);
        asserting!("bits(RangeFrom) is -1 (0xff) when completely past the last bit")
            .that(&byte.bits(8..))
            .is_equal_to(-1);

        asserting!("bits(Range) can index past the last bit")
            .that(&byte.bits(4..16))
            .is_equal_to(-6);
        asserting!("bits(Range) is -1 (0xff) when completely past the last bit")
            .that(&byte.bits(8..16))
            .is_equal_to(-1);

        asserting!("bits(RangeInclusive) can index past the last bit")
            .that(&byte.bits(4..=15))
            .is_equal_to(-6);
        asserting!("bits(RangeInclusive) is -1 (0xff) when completely past the last bit")
            .that(&byte.bits(8..=15))
            .is_equal_to(-1);

        asserting!("bits(RangeEU) can index past the last bit")
            .that(&byte.bits(RangeEU(4)))
            .is_equal_to(-3);
        asserting!("bits(RangeEU) is -1 (0xff) when completely past the last bit")
            .that(&byte.bits(RangeEU(8)))
            .is_equal_to(-1);

        asserting!("bits(RangeEE) can index past the last bit")
            .that(&byte.bits(RangeEE(4, 16)))
            .is_equal_to(-3);
        asserting!("bits(RangeEE) is -1 (0xff) when completely past the last bit")
            .that(&byte.bits(RangeEE(8, 17)))
            .is_equal_to(-1);

        asserting!("bits(RangeEI) can index past the last bit")
            .that(&byte.bits(RangeEI(4, 15)))
            .is_equal_to(-3);
        asserting!("bits(RangeEI) is -1 (0xff) when completely past the last bit")
            .that(&byte.bits(RangeEI(8, 16)))
            .is_equal_to(-1);
    }
}
