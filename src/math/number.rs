use num_traits::{Float, FromPrimitive, Num, ToPrimitive};
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

/// Basic number trait
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
    + FromPrimitive
    + ToPrimitive
{
}

impl<T> Number for T where
    T: Copy
        + Clone
        + Debug
        + Num
        + PartialOrd
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + FromPrimitive
        + ToPrimitive
{
}

/// Basic float number trait
pub trait FloatNumber: Number + Float {}

impl<T> FloatNumber for T where T: Number + Float {}
