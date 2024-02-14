use std::ops::{Add, Neg};

#[derive(Clone)]
pub struct BigUint {
    i: Vec<u64>
}

impl BigUint {
    fn new(size: usize) -> Self {
        Self {
            i: vec![u64::default(); size]
        }
    }

    pub fn to_binary(&self) -> String {
        let mut res = String::with_capacity(8 * self.i.len());
        for i in self.i.iter() {
            res += &format!("{:064b}", i);
        }
        res
    }

    pub fn extend(mut self) -> Self {
        if let Some(i) = self.i.get(0) {
            self.i.insert(0, 0);
        } else {
            self.i.push(0);
        }
        self
    }
}

impl From<u64> for BigUint {
    fn from(value: u64) -> Self {
        Self {
            i: vec![value]
        }
    }
}

impl From<u128> for BigUint {
    fn from(value: u128) -> Self {
        Self {
            i: vec![(value >> 64) as u64, value as u64]
        }
    }
}

impl Default for BigUint {
    fn default() -> Self {
        Self {
            i: Vec::new(),
        }
    }
}

// impl Neg for BigUint {
 //    type Output = Self;
 //    fn neg(mut self) -> Self::Output {
 //        for i in self.i.iter_mut() {
 //            *i = !(*i);
 //        }
// 
 //        let mut carry = false;
// 
 //        if let Some(i) = self.i.last_mut() {
 //            let (res, c) = i.overflowing_add(1);
 //            *i = res;
 //            carry = c;
 //        }
 //        for i in self.i.iter_mut().rev().skip(1) {
 //            if carry {
 //                let (rres, ss_carry) = i.overflowing_add(1);
 //                *i = rres;
 //                if !ss_carry {carry = false}
 //            } else {
 //                break;
 //            }
 //        }
// 
 //        self
 //    }
//} 

impl Add<BigUint> for BigUint {
    type Output = BigUint;
    fn add(self, rhs: BigUint) -> Self::Output {
        let (high, mut low) = if self.i.len() >= rhs.i.len() {(self, rhs)} else {(rhs, self)};
        let mut result = BigUint::new(high.i.len());
        let d = high.i.len() - low.i.len();

        for i in 0..d {
            low = low.extend();
        }

        let mut carry = false;
        for (ind, (h, l)) in high.i.iter().zip(low.i.iter()).enumerate() {
            let (res, c) = h.overflowing_add(*l);
            if carry {
                let (res, c) = res.overflowing_add(1);
                result.i[ind] = res;
                if !c {carry = false;}
            } else {
                result.i[ind] = res;
            }
            if c {
                carry = true;
            }
        }

        if carry {
            result.i.insert(0, 1);
        }

        result
    }
}

use std::ops::AddAssign;

impl AddAssign<BigUint> for BigUint {
    fn add_assign(&mut self, rhs: BigUint) {
        *self = self.clone() + rhs;
    }
}

use std::ops::Mul;
impl Mul<BigUint> for BigUint {
    type Output = BigUint;
    fn mul(self, rhs: BigUint) -> Self::Output {
        let (high, mut low) = if self.i.len() >= rhs.i.len() {(self, rhs)} else {(rhs, self)};
        let mut result = BigUint::new(high.i.len());
        let d = high.i.len() - low.i.len();

        for i in 0..d {
            low = low.extend();
        }

        let mut offset = 0;
        let mut carry = 0;
        let mut intermediates = Vec::new();
        for (ind, i) in low.i.iter().enumerate().rev() {
            let mut intermediate: Vec<u64> = Vec::with_capacity(high.i.len());
            for i in 0..offset { intermediate.push(0) }
            for j in high.i.iter().rev() {
                let (upper, lower) = wide_multiply(*j, *i); 
                if carry != 0 {
                    let (r, c) = lower.overflowing_add(carry);
                    intermediate.push(r);
                    if !c {carry = 0} else {carry = 1}
                } else {
                    intermediate.push(lower);
                }
                carry += upper;
            }
            if carry != 0 { intermediate.push(carry) }
            if let Some(a) = intermediate.last() {
                if *a == 0 {
                   intermediate.pop();
                }
            }
            let mut num = BigUint::new(intermediate.len());
            for (ind, i) in intermediate.iter().rev().enumerate() {
                num.i[ind] = *i;
            }
            offset += 1;
            intermediates.push(num);
        }

        for i in intermediates {
            result += i;
        }

        return result;
    }
}

pub fn wide_multiply(x: u64, y: u64) -> (u64, u64) {
    let ((x, y), (z, w)) = (high_low(x), high_low(y));
    
    let (yw_high, yw_low) = high_low(y * w);
    let (xw_high, xw_low) = high_low(x * w);
    let (yz_high, yz_low) = high_low(y * z);
    let (xz_high, xz_low) = high_low(x * z);
    let (mut upper, mut lower) = (0, 0);
    upper = x * z;
    lower = y * w; 
    let (r, c) = lower.overflowing_add(xw_low << 32);
    if c { upper += 1 };

    let (r, c) = r.overflowing_add(yz_low << 32);
    if c { upper += 1 };
    lower = r;

    upper += yz_high;
    upper += xw_high;


    
    


    (upper, lower)
}

pub fn u64_left(i: u64) -> u64 {
    //println!("\n====    ====    ====    ====   =");
    //println!("\n{:064b}\n{:064b}\n", i, i >> 32);
    i >> 32
}

pub fn u64_right(i: u64) -> u64 {
    //println!("\n====    ====    ====    ====   =");
    //println!("\n{:064b}\n{:064b}\n", i, i & ((u32::MAX as u64)));
    i & ((u32::MAX as u64))
}

pub fn high_low(i: u64) -> (u64, u64) {
    (u64_left(i), u64_right(i))
}
