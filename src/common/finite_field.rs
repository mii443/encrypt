use std::{ops::{Add, Sub, Mul, AddAssign, SubAssign, Div, Rem}, fmt::Debug};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FiniteFieldElement<T>
where
    T: PartialEq + Copy
{
    pub value: T,
    pub p: T,
    pub one: T
}

impl<T> FiniteFieldElement<T>
where
    T: PartialEq + Copy
{
    pub fn new(value: T, p: T, one: T) -> Self {
        Self { value, p, one }
    }
}

impl<T> FiniteFieldElement<T>
where
    T: Rem<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + PartialEq + PartialOrd + Copy + Debug
{
    fn pow(self, e: T) -> Self {
        let one = self.one;
        let two = one + one;
        let mut r = Self::new(one, self.p, self.one);
        let mut i = e;
        let zero = self.value - self.value;
        while i > zero {
            if i % two != zero {
                r = r * self;
            }
            i = i / two;
        }
        r * self
    }
}

impl<T> Add for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Cannot add different field value.")
        }
        let sum = self.value + rhs.value;
        if sum >= self.p {
            Self::new(sum - self.p, self.p, self.one)
        } else {
            Self::new(sum, self.p, self.one)
        }
    }
}

impl<T> AddAssign for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self) + rhs
    }
}

impl<T> Sub for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Cannot sub different field value.");
        }
        if self.value < rhs.value {
            Self::new(self.p - rhs.value + self.value , self.p, self.one)
        } else {
            Self::new(self.value - rhs.value, self.p, self.one)
        }
    }
}

impl<T> SubAssign for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = (*self) - rhs
    }
}

impl<T> Mul for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Copy + Debug
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Cannot mul different field value.");
        }
        let one = self.one;
        let two = one + one;
        let zero = self.value - self.value;
        let mut tmp = self;
        let mut i = rhs.value;
        let mut r = Self::new(zero, self.p, self.one);
        while i > zero {
            if i % two != zero {
                r += tmp;
            }
            i = i / two;
            tmp = tmp + tmp;
        }
        r
    }
}

impl<T> Div for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Div<Output = T> + PartialEq + PartialOrd + Copy + Debug
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let one = self.one;
        self * rhs.pow(self.p - one - one)
    }
}

#[cfg(test)]
mod tests {
    use primitive_types::U512;

    use super::FiniteFieldElement;

    #[test]
    fn add() {
        let a = FiniteFieldElement::new(U512::from(2), U512::from(7), U512::from(1));
        let b = FiniteFieldElement::new(U512::from(1), U512::from(7), U512::from(1));
        let c = FiniteFieldElement::new(U512::from(3), U512::from(7), U512::from(1));
        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = FiniteFieldElement::new(U512::from(6), U512::from(7), U512::from(1));
        let b = FiniteFieldElement::new(U512::from(4), U512::from(7), U512::from(1));
        let c = FiniteFieldElement::new(U512::from(2), U512::from(7), U512::from(1));
        assert_eq!(a - b, c);
    }

    #[test]
    fn mul() {
        let a = FiniteFieldElement::new(U512::from(3), U512::from(13), U512::from(1));
        let b = FiniteFieldElement::new(U512::from(12), U512::from(13), U512::from(1));
        let c = FiniteFieldElement::new(U512::from(10), U512::from(13), U512::from(1));
        assert_eq!(a * b, c);
    }

    #[test]
    fn pow() {
        let a = FiniteFieldElement::new(U512::from(3), U512::from(13), U512::from(1));
        let b = FiniteFieldElement::new(U512::from(1), U512::from(13), U512::from(1));
        assert_eq!(a.pow(U512::from(3)), b);
    }

    #[test]
    fn div() {
        let a = FiniteFieldElement::new(U512::from(7), U512::from(19), U512::from(1));
        let b = FiniteFieldElement::new(U512::from(5), U512::from(19), U512::from(1));
        let c = FiniteFieldElement::new(U512::from(9), U512::from(19), U512::from(1));
        assert_eq!(a / b, c);
    }
}