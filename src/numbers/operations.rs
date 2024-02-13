pub trait Multiplication: Mul<Self, Output = Self> + Sized {
    fn multiplicative_identity() -> Self;
}

pub trait Addition: Add<Self, Output = Self> + Sized {
    fn additive_identity() -> Self;
}

pub trait Subtraction: Sub<Self, Output = Self> + Sized {}

pub trait Division: Div<Self, Output = Self> + Sized {}

pub trait IntegerDivision: Div<Self, Output = Self> + Sized {}

pub trait Remainder: Rem<Self, Output = Self> + Sized {}


use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::macros::{implement_constant_function, implement_empty};

implement_constant_function!(Multiplication, multiplicative_identity, 1, i8, u8, i16, u16, i32, u32, isize, usize, i64, u64, i128, u128);
implement_constant_function!(Addition, additive_identity, 0, i8, u8, i16, u16, i32, u32, isize, usize, i64, u64, i128, u128);


implement_constant_function!(Multiplication, multiplicative_identity, 1.0, f32, f64);
implement_constant_function!(Addition, additive_identity, 0.0, f32, f64);
implement_empty!(Subtraction, i8, u8, i16, u16, i32, u32, isize, usize, i64, u64, i128, u128);
implement_empty!(IntegerDivision, i8, u8, i16, u16, i32, u32, isize, usize, i64, u64, i128, u128);
implement_empty!(Remainder, i8, u8, i16, u16, i32, u32, isize, usize, i64, u64, i128, u128);
implement_empty!(Subtraction, f32, f64);
implement_empty!(Division, f32, f64);
