use std::ops::{Add, Mul, Sub};
use crate::numbers::{Addition, Multiplication};

pub trait Integer:
    Addition + Multiplication + Subtraction + IntegerDivision + Remainder + Eq
    + Sized + std::fmt::Display + Clone
{
    fn gcd_euclidean(self, rhs: Self) -> Self {
        if rhs == Self::additive_identity() {
            self
        } else {
            rhs.clone().gcd_euclidean(self % rhs)
        }
    }
}

use crate::macros::implement_empty;

use super::operations::{IntegerDivision, Remainder, Subtraction};
implement_empty!(Integer, i8, u8, i16, u16, i32, u32, isize, usize, i64, u64, i128, u128);
