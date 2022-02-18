use std::{ops::{Add, Sub, Mul}, fmt::{self, Display}};

use bigdecimal::{BigDecimal, Zero};

#[macro_export]
macro_rules! b {
    ( $x: expr ) => {
        BigDecimal::from_i64($x).unwrap()
    };
}

#[macro_export]
macro_rules! ffe {
    ( $value: expr, $p: expr ) => {
        FiniteFieldElement::new(b!($value), b!($p))
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct FiniteFieldElement {
    pub value: BigDecimal,
    p: BigDecimal
}

fn pmod(x: BigDecimal, y: BigDecimal) -> BigDecimal {
    if x < BigDecimal::zero() {
        (y.clone() - (-x % y.clone())) % y
    } else {
        x % y
    }
}

impl FiniteFieldElement {
    pub fn new(value: BigDecimal, p: BigDecimal) -> Self {
        Self { value: pmod(value, p.clone()), p }
    }
}

impl Display for FiniteFieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} mod {}", self.value, self.p)
    }
}

impl Add for FiniteFieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("p doesn't match: {} != {}", self.p, rhs.p);
        }
        Self::new(self.value + rhs.value, self.p)
    }
}

impl Sub for FiniteFieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = FiniteFieldElement::new(-rhs.value, rhs.p);
        self + rhs
    }
}

impl Mul for FiniteFieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("p doesn't match: {} != {}", self.p, rhs.p);
        }
        Self::new(self.value * rhs.value, self.p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::{BigDecimal, FromPrimitive};

    #[test]
    fn add() {
        let a = ffe!(2, 5);
        let b = ffe!(4, 5);
        assert_eq!(a + b, ffe!(1, 5));
    }

    #[test]
    fn sub() {
        let a = ffe!(2, 5);
        let b = ffe!(3, 5);
        assert_eq!(a - b, ffe!(4, 5));
    }

    #[test]
    fn mul() {
        let a = ffe!(2, 5);
        let b = ffe!(3, 5);
        assert_eq!(a * b, ffe!(1, 5));
    }
}