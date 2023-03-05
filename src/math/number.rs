use num_traits::real::Real;
use num_traits::Num;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

/// Trait for clamp operation.
pub trait Clamp {
    /// Clamp value to be within the range [min, max].
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;
}

/// Trait for integer number.
pub trait Number:
    Copy
    + Clone
    + Debug
    + Num
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Clamp
{
    /// Create value of self type from an u8 number.
    #[must_use]
    fn from_u8(n: u8) -> Self;

    /// Create value of self type from an u32 number.
    #[must_use]
    fn from_u32(n: u32) -> Self;

    /// Create value of self type from an u64 number.
    #[must_use]
    fn from_u64(n: u64) -> Self;

    /// Create value of self type from an usize number.
    #[must_use]
    fn from_usize(n: usize) -> Self;
}

/// Trait for float number.
pub trait Float: Number + Real {
    /// Create value of self type from an f32 number.
    #[must_use]
    fn from_f32(n: f32) -> Self;

    /// Create value of self type from an f64 number.
    #[must_use]
    fn from_f64(n: f64) -> Self;
}

macro_rules! impl_clamp {
    ($number:ty) => {
        impl Clamp for $number {
            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self {
                assert!(min <= max);
                if self < min {
                    min
                } else if self > max {
                    max
                } else {
                    self
                }
            }
        }
    };
}

macro_rules! impl_number {
    ($number:ty) => {
        impl Number for $number {
            #[inline]
            fn from_u8(n: u8) -> Self {
                n as $number
            }

            #[inline]
            fn from_u32(n: u32) -> Self {
                n as $number
            }

            #[inline]
            fn from_u64(n: u64) -> Self {
                n as $number
            }

            #[inline]
            fn from_usize(n: usize) -> Self {
                n as $number
            }
        }
    };
}

macro_rules! impl_float {
    ($number:ty) => {
        impl Float for $number {
            #[inline]
            fn from_f32(n: f32) -> Self {
                n as $number
            }

            #[inline]
            fn from_f64(n: f64) -> Self {
                n as $number
            }
        }
    };
}

impl_clamp!(u8);
impl_clamp!(u16);
impl_clamp!(u32);
impl_clamp!(u64);
impl_clamp!(u128);
impl_clamp!(f32);
impl_clamp!(f64);

impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(u128);
impl_number!(f32);
impl_number!(f64);

impl_float!(f32);
impl_float!(f64);
