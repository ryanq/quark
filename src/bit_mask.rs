use crate::BitSize;

/// Provides bit mask calculation and masking on values.
///
/// This trait defines type methods for generating a bit mask of a specified length and value
/// methods for masking to a specified bit length.
///
/// # Examples
///
/// ```
/// use quark::BitMask;
///
/// let nibble_mask = u32::mask(4);
/// assert_eq!(nibble_mask, 0xf);
///
/// let value: u32 = 0x1234_5678;
/// assert_eq!(value.mask_to(12), 0x678);
/// ```
pub trait BitMask: BitSize {
    /// Returns a mask with the requested number of bits set.
    fn mask(size: usize) -> Self;

    /// Masks the value to the requested number of bits.
    fn mask_to(&self, size: usize) -> Self;
}

macro_rules! bit_mask_impl {
    ($type:ty, _, $s_ty:ty) => {
        bit_mask_impl!($type, $type, $s_ty);
    };
    ($type:ty, $u_ty:ty, _) => {
        bit_mask_impl!($type, $u_ty, $type);
    };
    ($type:ty, $u_ty:ty, $s_ty:ty) => {
        impl BitMask for $type {
            fn mask(size: usize) -> Self {
                let high_bit = ((1 as $u_ty) << (Self::BIT_SIZE - 1)) as $s_ty;
                match size {
                    0 => 0,
                    s if s < Self::BIT_SIZE => {
                        (((high_bit >> s) as $u_ty) >> Self::BIT_SIZE - s) as Self
                    }
                    _ => (high_bit >> Self::BIT_SIZE - 1) as Self,
                }
            }

            fn mask_to(&self, size: usize) -> Self {
                self & Self::mask(size)
            }
        }
    };
}

bit_mask_impl!(u8, _, i8);
bit_mask_impl!(u16, _, i16);
bit_mask_impl!(u32, _, i32);
bit_mask_impl!(u64, _, i64);
bit_mask_impl!(u128, _, i128);
bit_mask_impl!(usize, _, isize);
bit_mask_impl!(i8, u8, _);
bit_mask_impl!(i16, u16, _);
bit_mask_impl!(i32, u32, _);
bit_mask_impl!(i64, u64, _);
bit_mask_impl!(i128, u128, _);
bit_mask_impl!(isize, usize, _);

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn calculating_masks() {
        asserting!("masks of length 0 have no bits set")
            .that(&u8::mask(0))
            .is_equal_to(0);

        asserting!("masks have the correct number of bits set")
            .that(&[u8::mask(7), u8::mask(4), u8::mask(3)])
            .is_equal_to(&[0x07f, 0x0f, 0x07]);

        asserting!("masks longer than the bit length have all bits set")
            .that(&u8::mask(10))
            .is_equal_to(0xff);
    }

    #[test]
    fn masking_values() {
        asserting!("masking to 0 bits returns 0")
            .that(&145u8.mask_to(0))
            .is_equal_to(0);

        asserting!("masking to a number truncates the value")
            .that(&255u8.mask_to(4))
            .is_equal_to(15);

        asserting!("masking to more bits than the bit length returns the number")
            .that(&204u8.mask_to(10))
            .is_equal_to(204);
    }

    #[test]
    fn signed_masks() {
        asserting!("signed masks are the same as unsigned masks")
            .that(&(u8::mask(4), i8::mask(4)))
            .is_equal_to((0x0f, 0x0f));
    }
}
