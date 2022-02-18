use std::{ops::Add, fmt::{self, Display}};

use bigdecimal::{BigDecimal, Zero};

#[derive(Debug, Clone)]
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
