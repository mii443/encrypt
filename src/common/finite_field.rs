use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use bigdecimal::{num_bigint::BigInt, Num};
use primitive_types::U512;
use serde::{Deserialize, Serialize};

use super::math::{mod_inv, plus_mod};

#[derive(PartialEq, Debug, Copy, Clone, PartialOrd, Deserialize, Serialize)]
pub struct FiniteFieldElement {
    pub value: U512,
    pub p: U512,
}

impl FiniteFieldElement {
    pub fn new(value: U512, p: U512) -> Self {
        Self { value, p }
    }

    pub fn inverse(&self) -> Self {
        let left = BigInt::from_str_radix(&format!("{}", self.value), 10).unwrap();
        let right = BigInt::from_str_radix(&format!("{}", self.p), 10).unwrap();
        Self::new(
            U512::from_str_radix(&format!("{}", mod_inv(left, right)), 10).unwrap(),
            self.p,
        )
    }
}

impl FiniteFieldElement {
    pub fn pow(self, e: U512) -> Self {
        let k = e.bits();
        let mut a1 = self;
        let mut a2 = self * self;
        let mut i = (k - 2) as i64;

        while i >= 0 {
            if e.bit(i as usize) {
                a1 = a1 * a2;
            } else {
                a2 = a1 * a2;
                a1 = a1 * a1;
            }
            i -= 1;
        }
        a1
    }
}

impl Display for FiniteFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for FiniteFieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Cannot add different field value.")
        }
        let sum = self.value + rhs.value;
        if sum >= self.p {
            Self::new(sum - self.p, self.p)
        } else {
            Self::new(sum, self.p)
        }
    }
}

impl AddAssign for FiniteFieldElement {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self) + rhs
    }
}

impl Sub for FiniteFieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Cannot sub different field value.");
        }
        if self.value < rhs.value {
            Self::new(self.p - rhs.value + self.value, self.p)
        } else {
            Self::new(self.value - rhs.value, self.p)
        }
    }
}

impl SubAssign for FiniteFieldElement {
    fn sub_assign(&mut self, rhs: Self) {
        *self = (*self) - rhs
    }
}

impl Mul for FiniteFieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Cannot mul different field value.");
        }
        let mut tmp = self;
        let mut i = rhs.value;
        let mut r = Self::new(U512::from(0), self.p);
        while i > U512::from(0) {
            if i & U512::from(1) == U512::from(1) {
                r += tmp;
            }
            i = i >> 1;
            tmp = tmp + tmp;
        }
        r
    }
}

impl Div for FiniteFieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let left = BigInt::from_str_radix(&format!("{}", rhs.value), 10).unwrap();
        let right = BigInt::from_str_radix(&format!("{}", rhs.p), 10).unwrap();
        let mod_inv = U512::from_str_radix(&format!("{}", mod_inv(left, right)), 10).unwrap();
        self * FiniteFieldElement::new(mod_inv, rhs.p)
    }
}

impl Neg for FiniteFieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let value = -BigInt::from_str_radix(&format!("{}", self.value), 10).unwrap();
        let p = BigInt::from_str_radix(&format!("{}", self.p), 10).unwrap();
        let plus_mod = plus_mod(value, p);
        FiniteFieldElement::new(
            U512::from_str_radix(&format!("{}", plus_mod), 10).unwrap(),
            self.p,
        )
    }
}

#[cfg(test)]
mod tests {
    use primitive_types::U512;

    use super::FiniteFieldElement;

    #[test]
    fn add() {
        let a = FiniteFieldElement::new(U512::from(2u8), U512::from(7u8));
        let b = FiniteFieldElement::new(U512::from(1u8), U512::from(7u8));
        let c = FiniteFieldElement::new(U512::from(3u8), U512::from(7u8));
        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = FiniteFieldElement::new(U512::from(6u8), U512::from(7u8));
        let b = FiniteFieldElement::new(U512::from(4u8), U512::from(7u8));
        let c = FiniteFieldElement::new(U512::from(2u8), U512::from(7u8));
        assert_eq!(a - b, c);
    }

    #[test]
    fn mul() {
        let a = FiniteFieldElement::new(U512::from(3u8), U512::from(13u8));
        let b = FiniteFieldElement::new(U512::from(12u8), U512::from(13u8));
        let c = FiniteFieldElement::new(U512::from(10u8), U512::from(13u8));
        assert_eq!(a * b, c);
    }

    #[test]
    fn pow() {
        let a = FiniteFieldElement::new(U512::from(3u8), U512::from(13u8));
        let b = FiniteFieldElement::new(U512::from(1u8), U512::from(13u8));
        assert_eq!(a.pow(U512::from(3)), b);
    }

    #[test]
    fn div() {
        let a = FiniteFieldElement::new(U512::from(7u8), U512::from(19u8));
        let b = FiniteFieldElement::new(U512::from(5u8), U512::from(19u8));
        let c = FiniteFieldElement::new(U512::from(9u8), U512::from(19u8));
        assert_eq!(a / b, c);
    }
}
