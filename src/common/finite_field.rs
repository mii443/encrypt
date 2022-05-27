use std::{ops::{Add, Sub, Mul, AddAssign, SubAssign, Div, Rem}, fmt::Debug};

use primitive_types::U512;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FiniteFieldElement {
    pub value: U512,
    pub p: U512 
}

impl FiniteFieldElement {
    pub fn new(value: U512, p: U512) -> Self {
        Self { value, p }
    }
}

impl FiniteFieldElement {
    fn pow(self, e: U512) -> Self {
        let k = e.bits();
        let mut a1 = self.value;
        let mut a2 = self.value * self.value;
        let mut i = (k - 2) as i64;

        while i >= 0 {
            if e.bit(i as usize) {
                a1 = (a1 * a2) % self.p;
            } else {
                a2 = (a1 * a2) % self.p;
                a1 = (a1 * a1) % self.p;
            }
            i -= 1;
        }
        Self::new(a1, self.p)
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
            Self::new(self.p - rhs.value + self.value , self.p)
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
        let rhs = rhs.pow(self.p - U512::from(2));
        println!("{:?} * {:?}", self, rhs);
        self * rhs
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

        println!("{:?}", FiniteFieldElement::new(U512::from(5u8), U512::from(19u8)).pow(U512::from(17)));
    }

    #[test]
    fn div() {
        let a = FiniteFieldElement::new(U512::from(7u8), U512::from(19u8));
        let b = FiniteFieldElement::new(U512::from(5u8), U512::from(19u8));
        let c = FiniteFieldElement::new(U512::from(9u8), U512::from(19u8));
        assert_eq!(a / b, c);
    }
}
