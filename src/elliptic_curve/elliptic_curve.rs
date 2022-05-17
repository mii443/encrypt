use std::ops::{Div, Add, Sub, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticCurve<T> {
    pub a: T,
    pub b: T
}

impl<T> EllipticCurve<T> {
    pub fn point(self, x: T, y: T) -> EllipticCurvePoint<T> {
        EllipticCurvePoint::Point { x, y, a: self.a, b: self.b }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EllipticCurvePoint<T> {
    Point {
        x: T,
        y: T,
        a: T,
        b: T
    },
    Infinity
}

impl<T> Add for EllipticCurvePoint<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + PartialEq
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self.clone() {
            EllipticCurvePoint::Point { x: x1, y: y1, a, b } => {
                match rhs {
                    EllipticCurvePoint::Point { x: x2, y: y2, a: a2, b: b2 } => {

                        if a != a2 || b != b2 {
                            panic!("Cannot add different curve point.");
                        }

                        if x1 == x2 && y2 == y1 - y1 - y1 {
                            return EllipticCurvePoint::Infinity
                        }

                        let one = b / b;
                        let two = one + one;
                        let three = two + one;
                        let l = if x1 == x2 && y1 == y2 {
                            (x1 * x1 * three + a) / (y1 * two)
                        } else {
                            (y2 - y1) / (x2 - x1)
                        };
                        let x = l * l - x1 - x2;
                        let y = l * (x1 - x) - y1;

                        EllipticCurvePoint::Point { x, y, a, b }
                    },
                    EllipticCurvePoint::Infinity => self
                }
            },
            EllipticCurvePoint::Infinity => rhs
        }
    }
}

impl<T, U> Mul<U> for EllipticCurvePoint<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + PartialEq,
    U: Sub<Output = U> + Div<Output = U> + Copy + PartialEq + PartialOrd
{
    type Output = Self;

    fn mul(self, rhs: U) -> Self::Output {
        let one = rhs / rhs;
        let zero = rhs - rhs;
        let mut n = rhs;
        let mut r: EllipticCurvePoint<T> = EllipticCurvePoint::Infinity;
        while n > zero {
            r = r + self;
            n = n - one;
        }

        r
    }
}
