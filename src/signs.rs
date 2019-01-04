use crate::BitSize;

/// Provides operations based on signs
///
/// This trait defines functions for querying the sign bit of values and for sign extending values
/// from arbitrary values.
///
/// # Examples
///
/// ```
/// use quark::Signs;
///
/// let value: u32 = 0xffff_fff0;
/// assert_eq!(value.sign_bit(), true);
/// 
/// let value: u32 = 0x0000_8000;
/// assert_eq!(value.sign_extend(16), 0xffff_8000);
/// ```
pub trait Signs: BitSize {
    /// Returns whether the sign bit is set.
    fn sign_bit(&self) -> bool;

    /// Fills the upper N bits of a value with the next bit down.
    fn sign_extend(&self, bits: usize) -> Self;
}

macro_rules! signs_impl {
    ($s_type:ty) => {
        impl Signs for $s_type {
            fn sign_bit(&self) -> bool {
                *self < 0
            }

            fn sign_extend(&self, bits: usize) -> Self {
                if bits >= Self::BIT_SIZE {
                    0
                } else {
                    self << bits >> bits
                }
            }
        }
    };
    ($u_type:ty, $s_type:ty) => {
        impl Signs for $u_type {
            fn sign_bit(&self) -> bool {
                (*self as $s_type) < 0
            }

            fn sign_extend(&self, bits: usize) -> Self {
                if bits >= Self::BIT_SIZE {
                    0
                } else {
                    (((self << bits) as $s_type) >> bits) as $u_type
                }
            }
        }
    };
}

signs_impl!(i8);
signs_impl!(i16);
signs_impl!(i32);
signs_impl!(i64);
signs_impl!(i128);
signs_impl!(isize);
signs_impl!(u8, i8);
signs_impl!(u16, i16);
signs_impl!(u32, i32);
signs_impl!(u64, i64);
signs_impl!(u128, i128);
signs_impl!(usize, isize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unsigned() {
        let value: u32 = 0x0000_8000;
        assert_eq!(value.sign_bit(), false);
        assert_eq!(value.sign_extend(15), 0x0000_8000);
        assert_eq!(value.sign_extend(16), 0xffff_8000);
        assert_eq!(value.sign_extend(17), 0);

        let value: u32 = 0x8000_8000;
        assert_eq!(value.sign_bit(), true);
        assert_eq!(value.sign_extend(15), 0x0000_8000);
        assert_eq!(value.sign_extend(16), 0xffff_8000);
        assert_eq!(value.sign_extend(17), 0);
    }

    #[test]
    fn signed() {
        let value: i32 = -65536; // 0xffff_0000
        assert_eq!(value.sign_bit(), true);
        assert_eq!(value.sign_extend(15), -65536);
        assert_eq!(value.sign_extend(16), 0);
        assert_eq!(value.sign_extend(17), 0);

        let value: i32 = 0x7fff_0000;
        assert_eq!(value.sign_bit(), false);
        assert_eq!(value.sign_extend(1), -65536);
        assert_eq!(value.sign_extend(15), -65536);
        assert_eq!(value.sign_extend(16), 0);
        assert_eq!(value.sign_extend(17), 0);
    }
}
