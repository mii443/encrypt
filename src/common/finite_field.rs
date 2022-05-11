use std::{ops::{Add, Sub, Mul, Neg}, fmt::{self, Display}};

use bigdecimal::{BigDecimal, Zero};

use super::math::{self, mod_inv, plus_mod};

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

#[macro_export]
macro_rules! ffeb {
    ( $value: expr, $p: expr ) => {
        FiniteFieldElement::new($value, $p)
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct FiniteFieldElement {
    pub value: BigDecimal,
    pub p: BigDecimal
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

    pub fn floor_div(self, rhs: FiniteFieldElement) -> Self {
        self.clone() * FiniteFieldElement { value: mod_inv(rhs.value, self.p.clone()), p:  self.p }
    }

    pub fn rem(self, rhs: FiniteFieldElement) -> FiniteFieldElement {
        FiniteFieldElement { value: plus_mod(self.value, rhs.value), p: self.p }
    }

    pub fn pow(self, rhs: FiniteFieldElement) -> FiniteFieldElement {
        FiniteFieldElement { value: plus_mod(math::pow(self.value, rhs.value), self.p.clone()), p: self.p }
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
        Self::new(self.value - rhs.value, rhs.p)
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

impl Neg for FiniteFieldElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.value, self.p)
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

    #[test]
    fn pow() {
        let a = ffe!(3, 50);
        let b = ffe!(4, 50);
        println!("{:?}", math::pow(a.clone().value, b.clone().value));
        assert_eq!(a.pow(b), ffe!(31, 50));
    }
}
