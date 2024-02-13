use std::ops::{Add, Div, Mul, Sub};

use crate::numbers::integer::Integer;

#[derive(Clone)]
pub struct Rational<T: Integer> {
    pub numerator: T,
    pub denominator: T,
}

impl <T: Integer> Rational<T> {
    pub fn new(numerator: T, denominator: T) -> Self {
        (numerator, denominator).into()
    }

    pub fn gcd(&self) -> T {
        self.numerator.clone().gcd_euclidean(self.denominator.clone())
    }

    pub fn reduce(self) -> Self {
        let gcd = self.gcd();
        Self::new(
            self.numerator.clone() / gcd.clone(),
            self.denominator.clone() / gcd
        )
    }
}

impl <T: Integer> Add<Rational<T>> for Rational<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Rational::new(
            self.numerator * rhs.denominator.clone() + rhs.numerator * self.denominator.clone(),
            self.denominator * rhs.denominator
        )
    }
}

impl <T: Integer> Add<T> for Rational<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Rational::new(
            self.numerator + rhs * self.denominator.clone(),
            self.denominator
        )
    }
}

impl <T: Integer> Sub<Rational<T>> for Rational<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Rational::new(
            self.numerator * rhs.denominator.clone() - rhs.numerator * self.denominator.clone(),
            self.denominator * rhs.denominator
        )
    }
}

impl <T: Integer> Sub<T> for Rational<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Rational::new(
            self.numerator - rhs * self.denominator.clone(),
            self.denominator
        )
    }
}

impl <T: Integer> Mul<Rational<T>> for Rational<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Rational::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator
        )
    }
}

impl <T: Integer> Mul<T> for Rational<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Rational::new(
            self.numerator * rhs,
            self.denominator
        )
    }
}

impl <T: Integer> Div<Rational<T>> for Rational<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Rational::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator
        )
    }
}

impl <T: Integer> Div<T> for Rational<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Rational::new(
            self.numerator,
            self.denominator * rhs
        )
    }
}

impl <T: Integer> std::fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl <T: Integer> From<T> for Rational<T> {
    fn from(value: T) -> Self {
        Self {
            numerator: value, denominator: T::additive_identity()
        }
    }
}

impl <T: Integer> From<(T, T)> for Rational<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            numerator: value.0, denominator: value.1
        }
    }
}