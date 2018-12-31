/// Provides the bit size of the type as a constant.
///
/// This trait defines a constant for the number of bits in the type. This constant can be useful
/// for implementing other traits on various sized types.
///
/// # Examples
///
/// ```
/// use quark::BitSize;
///
/// assert_eq!(u8::BIT_SIZE, 8);
/// assert_eq!(u128::BIT_SIZE, 128);
/// ```
pub trait BitSize {
    /// The size of the type in bits.
    const BIT_SIZE:usize;
}

macro_rules! bit_size_impl {
    ($type:ty, $sz:expr) => {
        impl BitSize for $type {
            const BIT_SIZE: usize = $sz;
        }
    };
}

bit_size_impl!(u8, 8);
bit_size_impl!(u16, 16);
bit_size_impl!(u32, 32);
bit_size_impl!(u64, 64);
bit_size_impl!(u128, 128);
#[cfg(target_pointer_width = "32")]
bit_size_impl!(usize, 32);
#[cfg(target_pointer_width = "64")]
bit_size_impl!(usize, 64);
bit_size_impl!(i8, 8);
bit_size_impl!(i16, 16);
bit_size_impl!(i32, 32);
bit_size_impl!(i64, 64);
bit_size_impl!(i128, 128);
#[cfg(target_pointer_width = "32")]
bit_size_impl!(isize, 32);
#[cfg(target_pointer_width = "64")]
bit_size_impl!(isize, 64);
