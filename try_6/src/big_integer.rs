use core::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Result},
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct BigInteger {
    base: Vec<u8>,
    sign: bool,
}

impl BigInteger {
    pub fn get_bit(&self, i: usize) -> u8 {
        self[i >> 3] >> i % 8 & 1
    }

    pub fn is_zero(&self) -> bool {
        self.iter().all(|&v| v == 0)
    }

    pub fn bit_length(&self) -> usize {
        match self.last() {
            Some(last) => (self.len() << 3) - last.leading_zeros() as usize,
            None => 0,
        }
    }
}

impl Add for BigInteger {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl AddAssign for BigInteger {
    fn add_assign(&mut self, mut rhs: Self) {
        if rhs.sign == self.sign {
            let Self { base, .. } = rhs;
            let mut rhs = base.into_iter();
            let mut this = self.iter_mut();
            if {
                let mut carry = false;
                while let Some(next) = this.next() {
                    let rhs = rhs.next().unwrap_or_default();
                    (*next, carry) = next.carrying_add(rhs, carry);
                }
                let mut value;
                while let Some(next) = rhs.next() {
                    (value, carry) = next.overflowing_add(carry.into());
                    self.push(value);
                }
                carry
            } {
                self.push(1);
            }
        } else {
            rhs.sign = self.sign;
            self.sub_assign(rhs);
        }
    }
}

impl Debug for BigInteger {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let sign = if self.sign { '+' } else { '-' };
        let mut iter = self.iter().rev();
        write!(f, "{sign}{:#04x}", iter.next().unwrap_or(&0))?;
        iter.try_for_each(|v| write!(f, "_{:02x}", v))?;
        Ok(())
    }
}

impl Deref for BigInteger {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BigInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Div for BigInteger {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl DivAssign for BigInteger {
    fn div_assign(&mut self, _rhs: Self) {
        todo!()
    }
}

impl From<i8> for BigInteger {
    fn from(value: i8) -> Self {
        let base = vec![value.unsigned_abs()];
        let sign = value.is_positive();
        Self { base, sign }
    }
}

impl From<i16> for BigInteger {
    fn from(value: i16) -> Self {
        let sign = value.is_positive();
        let mut base = Vec::new();
        let mut value = value.unsigned_abs();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign }
    }
}

impl From<i32> for BigInteger {
    fn from(value: i32) -> Self {
        let sign = value.is_positive();
        let mut base = Vec::new();
        let mut value = value.unsigned_abs();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign }
    }
}

impl From<i64> for BigInteger {
    fn from(value: i64) -> Self {
        let sign = value.is_positive();
        let mut base = Vec::new();
        let mut value = value.unsigned_abs();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign }
    }
}

impl From<i128> for BigInteger {
    fn from(value: i128) -> Self {
        let sign = value.is_positive();
        let mut base = Vec::new();
        let mut value = value.unsigned_abs();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign }
    }
}

impl From<isize> for BigInteger {
    fn from(value: isize) -> Self {
        let sign = value.is_positive();
        let mut base = Vec::new();
        let mut value = value.unsigned_abs();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign }
    }
}

impl From<u8> for BigInteger {
    fn from(value: u8) -> Self {
        Self {
            base: vec![value],
            sign: true,
        }
    }
}

impl From<u16> for BigInteger {
    fn from(mut value: u16) -> Self {
        let mut base = Vec::new();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign: true }
    }
}

impl From<u32> for BigInteger {
    fn from(mut value: u32) -> Self {
        let mut base = Vec::new();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign: true }
    }
}

impl From<u64> for BigInteger {
    fn from(mut value: u64) -> Self {
        let mut base = Vec::new();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign: true }
    }
}

impl From<u128> for BigInteger {
    fn from(mut value: u128) -> Self {
        let mut base = Vec::new();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign: true }
    }
}

impl From<usize> for BigInteger {
    fn from(mut value: usize) -> Self {
        let mut base = Vec::new();
        while value > 0 {
            base.push(value as u8);
            value >>= 8;
        }
        Self { base, sign: true }
    }
}

impl Mul for BigInteger {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl MulAssign for BigInteger {
    fn mul_assign(&mut self, _rhs: Self) {
        todo!()
    }
}

impl Ord for BigInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.sign.cmp(&other.sign) {
            Ordering::Equal => {
                let (lhs, rhs) = if self.sign {
                    (self, other)
                } else {
                    (other, self)
                };
                match lhs.len().cmp(&rhs.len()) {
                    Ordering::Equal => lhs.iter().rev().cmp(rhs.iter().rev()),
                    ord => ord,
                }
            }
            ord => ord,
        }
    }
}

impl PartialOrd for BigInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.sign.partial_cmp(&other.sign) {
            Some(Ordering::Equal) => {
                let (lhs, rhs) = if self.sign {
                    (self, other)
                } else {
                    (other, self)
                };
                match lhs.len().partial_cmp(&rhs.len()) {
                    Some(Ordering::Equal) => lhs.iter().rev().partial_cmp(rhs.iter().rev()),
                    ord => ord,
                }
            }
            ord => ord,
        }
    }
}

impl Sub for BigInteger {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl SubAssign for BigInteger {
    fn sub_assign(&mut self, mut rhs: Self) {
        if rhs.sign == self.sign {
            let Self { base, .. } = rhs;
            let mut rhs = base.into_iter();
            let mut this = self.iter_mut();
            if {
                let mut noborrow = true;
                while let Some(next) = this.next() {
                    let rhs = !rhs.next().unwrap_or_default();
                    (*next, noborrow) = next.carrying_add(rhs, noborrow);
                }
                while let Some(0) = self.last() {
                    self.pop();
                }
                let mut value;
                while let Some(b) = rhs.next() {
                    (value, noborrow) = (!b).overflowing_add(noborrow.into());
                    self.push(value);
                }
                !noborrow
            } {
                self.sign = !self.sign;
            }
        } else {
            rhs.sign = self.sign;
            self.add_assign(rhs);
        }
    }
}

#[test]
fn test_add() {
    assert_eq!(
        Add::add(BigInteger::from(u32::MAX), BigInteger::from(i32::MIN)),
        BigInteger::from(u32::MAX as i64 + i32::MIN as i64)
    );
}

#[test]
fn test_sub() {
    assert_eq!(
        Sub::sub(BigInteger::from(i32::MIN), BigInteger::from(u32::MAX)),
        BigInteger::from(i32::MIN as i64 - u32::MAX as i64)
    );
}
