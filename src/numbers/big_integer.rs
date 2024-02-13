use std::ops::{Add, Neg};

pub struct BigInteger {
    i: Vec<u64>
}

impl BigInteger {
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
            if i.clone() >> 63 == 1 {
                self.i.insert(0, u64::MAX);
            } else {
                self.i.insert(0, 0);
            }
        } else {
            self.i.push(0);
        }
        self
    }
}

impl From<u64> for BigInteger {
    fn from(value: u64) -> Self {
        Self {
            i: vec![value]
        }
    }
}

impl From<u128> for BigInteger {
    fn from(value: u128) -> Self {
        Self {
            i: vec![(value >> 64) as u64, value as u64]
        }
    }
}

impl Default for BigInteger {
    fn default() -> Self {
        Self {
            i: Vec::new(),
        }
    }
}

impl Neg for BigInteger {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for i in self.i.iter_mut() {
            *i = !(*i);
        }

        let mut carry = false;

        if let Some(i) = self.i.last_mut() {
            let (res, c) = i.overflowing_add(1);
            *i = res;
            carry = c;
        }
        for i in self.i.iter_mut().rev().skip(1) {
            if carry {
                let (rres, ss_carry) = i.overflowing_add(1);
                *i = rres;
                if !ss_carry {carry = false}
            } else {
                break;
            }
        }

        self
    }
}

impl Add<BigInteger> for BigInteger {
    type Output = BigInteger;
    fn add(self, rhs: BigInteger) -> Self::Output {
        let (high, mut low) = if self.i.len() >= rhs.i.len() {(self, rhs)} else {(rhs, self)};
        let mut result = BigInteger::new(high.i.len());
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